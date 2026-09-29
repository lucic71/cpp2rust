// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/ir_src.h"

#include <clang/AST/DeclCXX.h>
#include <clang/AST/DeclTemplate.h>
#include <clang/AST/ExprCXX.h>
#include <clang/AST/PrettyPrinter.h>
#include <clang/Lex/Lexer.h>

#include <algorithm>
#include <cassert>
#include <cstdlib>

#include "converter/converter_lib.h"
#include "logging.h"

namespace cpp2rust::IrSrc {

namespace {

using Kind = Node::Kind;
using llvm::cast;
using llvm::dyn_cast;

constexpr std::pair<Kind, const char *> kKindNames[] = {
    {Kind::kParam, "param"},         {Kind::kValue, "value"},
    {Kind::kBuiltin, "builtin"},     {Kind::kRecord, "record"},
    {Kind::kEnum, "enum"},           {Kind::kTypedef, "typedef"},
    {Kind::kConst, "const"},         {Kind::kVolatile, "volatile"},
    {Kind::kPointer, "ptr"},         {Kind::kLRef, "lref"},
    {Kind::kRRef, "rref"},           {Kind::kArray, "array"},
    {Kind::kFunctionType, "fntype"}, {Kind::kOpaque, "opaque"},
    {Kind::kFunction, "function"},   {Kind::kDecl, "decl"},
    {Kind::kMacro, "macro"},         {Kind::kUnary, "unary"},
    {Kind::kArrow, "arrow"},         {Kind::kPackParams, "pack_params"},
};

const char *kindName(Kind kind) {
  for (const auto &[k, name] : kKindNames) {
    if (k == kind) {
      return name;
    }
  }
  assert(0 && "unknown IR node kind");
  return "";
}

Kind kindFromName(llvm::StringRef name) {
  for (const auto &[k, n] : kKindNames) {
    if (name == n) {
      return k;
    }
  }
  llvm::errs() << "ERROR: unknown IR node kind '" << name << "'\n";
  std::exit(EXIT_FAILURE);
}

NodePtr ParseNodeJSON(const llvm::json::Value &value) {
  const auto *obj = value.getAsObject();
  assert(obj && "IR node must be an object");
  auto kind = kindFromName(*obj->getString("kind"));
  auto name = obj->getString("name").value_or("").str();
  NodePtr node;
  switch (kind) {
  case Kind::kParam:
    node = std::make_unique<ParamNode>(*obj->getInteger("param"));
    break;
  case Kind::kConst:
  case Kind::kVolatile:
    node = std::make_unique<QualNode>(kind, nullptr);
    break;
  case Kind::kPointer:
  case Kind::kLRef:
  case Kind::kRRef:
    node = std::make_unique<PointerNode>(kind, nullptr);
    break;
  case Kind::kArray:
    node = std::make_unique<ArrayNode>(nullptr);
    break;
  case Kind::kRecord:
    node = std::make_unique<RecordNode>(std::move(name));
    break;
  case Kind::kDecl:
    node = std::make_unique<DeclNode>(std::move(name));
    break;
  case Kind::kFunction:
  case Kind::kFunctionType: {
    auto fn = std::make_unique<FunctionNode>(kind, std::move(name));
    fn->variadic = obj->getBoolean("variadic").value_or(false);
    fn->is_const = obj->getBoolean("is_const").value_or(false);
    fn->is_volatile = obj->getBoolean("is_volatile").value_or(false);
    fn->ref = obj->getString("ref").value_or("").str();
    node = std::move(fn);
    break;
  }
  case Kind::kUnary:
    node = std::make_unique<UnaryNode>(std::move(name), nullptr);
    break;
  case Kind::kArrow:
    node = std::make_unique<ArrowNode>(nullptr, nullptr);
    break;
  default:
    node = std::make_unique<Node>(kind, std::move(name));
    break;
  }
  forEachField(*node, *node, [&](const char *key, auto &field, auto &) {
    if constexpr (kIsNodeList<decltype(field)>) {
      if (const auto *list = obj->getArray(key)) {
        for (const auto &child : *list) {
          field.push_back(ParseNodeJSON(child));
        }
      }
    } else if (const auto *child = obj->get(key)) {
      field = ParseNodeJSON(*child);
    }
    return true;
  });
  return node;
}

unsigned NumParams(const Node &ir) {
  unsigned n = 0;
  ir.forEachParam([&](unsigned param) { n = std::max(n, param); });
  assert(n <= Ir::kMaxGenerics && "template placeholder exceeds kMaxGenerics");
  return n;
}

ExprRule ParseExprRuleJSON(const llvm::json::Object &obj) {
  ExprRule rule;
  rule.ir = ParseNodeJSON(*obj.get("ir"));
  rule.num_params = NumParams(*rule.ir);
  if (const auto *init_type = obj.getObject("init_type")) {
    rule.init_type = InitTypeLocation{
        (unsigned)*init_type->getInteger("depth"),
        (unsigned)*init_type->getInteger("index"),
    };
  }
  return rule;
}

TypeRule ParseTypeRuleJSON(const llvm::json::Object &obj) {
  TypeRule rule;
  rule.ir = ParseNodeJSON(*obj.get("ir"));
  rule.num_params = NumParams(*rule.ir);
  return rule;
}

NodePtr qualify(NodePtr node, clang::QualType type) {
  if (type.isVolatileQualified()) {
    auto qualified = node->type.withVolatile();
    node = std::make_unique<QualNode>(Kind::kVolatile, std::move(node));
    node->type = qualified;
  }
  if (type.isConstQualified()) {
    auto qualified = node->type.withConst();
    node = std::make_unique<QualNode>(Kind::kConst, std::move(node));
    node->type = qualified;
  }
  return node;
}

std::string nameOf(const clang::NamedDecl *decl) {
  if (const auto *ctor = llvm::dyn_cast<clang::CXXConstructorDecl>(decl)) {
    return ctor->getParent()->getName().str();
  }
  if (const auto *dtor = llvm::dyn_cast<clang::CXXDestructorDecl>(decl)) {
    return "~" + dtor->getParent()->getName().str();
  }
  auto name = decl->getDeclName();
  if (name.getNameKind() == clang::DeclarationName::CXXConversionFunctionName) {
    return "operator conversion";
  }
  if (const auto *tag = llvm::dyn_cast<clang::TagDecl>(decl);
      tag && !tag->getIdentifier()) {
    if (const auto *tdef = tag->getTypedefNameForAnonDecl()) {
      return tdef->getName().str();
    }
    return "(anonymous)";
  }
  return name.getAsString();
}

std::string QualifiedName(const clang::NamedDecl *decl) {
  std::vector<std::string> parts{nameOf(decl)};
  for (const auto *dc = decl->getDeclContext(); dc; dc = dc->getParent()) {
    if (const auto *ns = llvm::dyn_cast<clang::NamespaceDecl>(dc)) {
      if (!ns->isInline() && !ns->isAnonymousNamespace()) {
        parts.push_back(ns->getName().str());
      }
    } else if (const auto *named = llvm::dyn_cast<clang::NamedDecl>(dc);
               named && (llvm::isa<clang::RecordDecl>(named) ||
                         llvm::isa<clang::FunctionDecl>(named))) {
      parts.push_back(nameOf(named));
    }
  }
  std::string out;
  for (auto it = parts.rbegin(); it != parts.rend(); ++it) {
    if (!out.empty()) {
      out += "::";
    }
    out += *it;
  }
  return out;
}

std::string tagName(const clang::TagDecl *tag) {
  if (!tag->getIdentifier() && !tag->getTypedefNameForAnonDecl()) {
    return GetNamedDeclAsString(tag);
  }
  if (tag->getIdentifier() && tag->getDeclContext()->isFunctionOrMethod()) {
    return GetNamedDeclAsString(tag);
  }
  if (auto renamed = DisambiguateAnonymousTag(tag); !renamed.empty()) {
    return renamed;
  }
  return QualifiedName(tag);
}

} // namespace

bool Node::operator==(const Node &other) const {
  return shallowEquals(other) &&
         zipChildren(*this, other, [](const NodePtr &a, const NodePtr &b) {
           return *a == *b;
         });
}

bool Node::shallowEquals(const Node &other) const {
  if (kind != other.kind || name != other.name) {
    return false;
  }
  if (const auto *param = dyn_cast<ParamNode>(this)) {
    return param->param == cast<ParamNode>(other).param;
  }
  if (const auto *fn = dyn_cast<FunctionNode>(this)) {
    const auto &other_fn = cast<FunctionNode>(other);
    return fn->variadic == other_fn.variadic &&
           fn->is_const == other_fn.is_const &&
           fn->is_volatile == other_fn.is_volatile && fn->ref == other_fn.ref;
  }
  return true;
}

std::string Node::indexKey() const {
  switch (kind) {
  case Kind::kFunction:
  case Kind::kDecl:
  case Kind::kRecord:
  case Kind::kEnum:
  case Kind::kTypedef:
  case Kind::kBuiltin:
    return name;
  case Kind::kMacro:
    return "macro:" + name;
  case Kind::kUnary:
    return "unary" + name + ":" + cast<UnaryNode>(this)->operand->indexKey();
  case Kind::kArrow:
    return cast<ArrowNode>(this)->member->indexKey();
  case Kind::kConst:
  case Kind::kVolatile:
    return cast<QualNode>(this)->operand->indexKey();
  case Kind::kPointer:
    return "*" + cast<PointerNode>(this)->pointee->indexKey();
  case Kind::kLRef:
    return "&" + cast<PointerNode>(this)->pointee->indexKey();
  case Kind::kRRef:
    return "&&" + cast<PointerNode>(this)->pointee->indexKey();
  case Kind::kArray:
    return "[]" + cast<ArrayNode>(this)->element->indexKey();
  default:
    return "";
  }
}

unsigned Node::specificity() const {
  unsigned n = kind == Kind::kParam ? 0 : 1;
  zipChildren(*this, *this, [&](const NodePtr &child, const NodePtr &) {
    n += child->specificity();
    return true;
  });
  return n;
}

bool Node::hasParam(unsigned n) const {
  if (const auto *param = dyn_cast<ParamNode>(this)) {
    return param->param == n;
  }
  return !zipChildren(*this, *this, [&](const NodePtr &child, const NodePtr &) {
    return !child->hasParam(n);
  });
}

void Node::forEachParam(const std::function<void(unsigned)> &fn) const {
  if (const auto *param = dyn_cast<ParamNode>(this)) {
    fn(param->param);
  }
  zipChildren(*this, *this, [&](const NodePtr &child, const NodePtr &) {
    child->forEachParam(fn);
    return true;
  });
}

std::string Node::str() const {
  if (const auto *param = dyn_cast<ParamNode>(this)) {
    return "T" + std::to_string(param->param);
  }
  if (const auto *qual = dyn_cast<QualNode>(this)) {
    return std::string(kindName(kind)) + " " + qual->operand->str();
  }
  std::string out;
  if (const auto *fn = dyn_cast<FunctionNode>(this)) {
    if (fn->is_const) {
      out += "const ";
    }
    if (fn->is_volatile) {
      out += "volatile ";
    }
  }
  out += kindName(kind);
  if (!name.empty()) {
    out += " " + name;
  }
  std::vector<std::string> fields;
  forEachField(*this, *this,
               [&](const char *key, const auto &field, const auto &) {
                 if constexpr (kIsNodeList<decltype(field)>) {
                   if (!field.empty()) {
                     std::string list;
                     for (const auto &child : field) {
                       list += (list.empty() ? "" : ", ") + child->str();
                     }
                     fields.push_back(std::string(key) + ": [" + list + "]");
                   }
                 } else if (field) {
                   fields.push_back(std::string(key) + ": " + field->str());
                 }
                 return true;
               });
  if (!fields.empty()) {
    out += "(";
    for (size_t i = 0; i < fields.size(); ++i) {
      out += (i ? ", " : "") + fields[i];
    }
    out += ")";
  }
  return out;
}

llvm::json::Value ToJSON(const Node &node) {
  llvm::json::Object obj{{"kind", kindName(node.kind)}};
  if (!node.name.empty()) {
    obj["name"] = node.name;
  }
  if (const auto *param = dyn_cast<ParamNode>(&node)) {
    obj["param"] = param->param;
  }
  if (const auto *fn = dyn_cast<FunctionNode>(&node)) {
    if (fn->is_const) {
      obj["is_const"] = true;
    }
    if (fn->is_volatile) {
      obj["is_volatile"] = true;
    }
    if (fn->variadic) {
      obj["variadic"] = true;
    }
    if (!fn->ref.empty()) {
      obj["ref"] = fn->ref;
    }
  }
  forEachField(node, node,
               [&](const char *key, const auto &field, const auto &) {
                 if constexpr (kIsNodeList<decltype(field)>) {
                   if (!field.empty()) {
                     llvm::json::Array list;
                     for (const auto &child : field) {
                       list.push_back(ToJSON(*child));
                     }
                     obj[key] = std::move(list);
                   }
                 } else if (field) {
                   obj[key] = ToJSON(*field);
                 }
                 return true;
               });
  return obj;
}

void ExprRule::dump() const {
  log() << "Matching: " << ir->str() << '\n';
  if (init_type.valid()) {
    log() << "  init type: depth " << init_type.depth << ", index "
          << init_type.index << '\n';
  }
}

void TypeRule::dump() const { log() << "name: " << ir->str() << '\n'; }

Rules Load(const std::filesystem::path &dir) {
  Rules rules;
  Ir::LoadJSON(rules, dir / "ir_src.json", true, ParseExprRuleJSON,
               ParseTypeRuleJSON);
  return rules;
}

NodePtr Builder::FromType(clang::QualType type) { return fromType(type, true); }

NodePtr Builder::fromType(clang::QualType type, bool top) {
  if (keep_builtin_typedef && top) {
    if (const auto *decltype_type =
            llvm::dyn_cast<clang::DecltypeType>(type.getTypePtr())) {
      type = decltype_type->getUnderlyingType();
    }
    auto canonical = type.getCanonicalType();
    const clang::NamedDecl *name = nullptr;
    if (const auto *typedef_type = type->getAs<clang::TypedefType>();
        typedef_type && canonical->isBuiltinType()) {
      name = typedef_type->getDecl();
    }
    if (name) {
      auto node = std::make_unique<Node>(Kind::kTypedef, name->getName().str());
      node->type = canonical.getUnqualifiedType();
      return qualify(std::move(node), canonical);
    }
    if (const auto *predef = type->getAs<clang::PredefinedSugarType>()) {
      auto node = std::make_unique<Node>(
          Kind::kTypedef, predef->getIdentifier()->getName().str());
      node->type = canonical.getUnqualifiedType();
      return qualify(std::move(node), canonical);
    }
    if (const auto *ptr = type->getAs<clang::PointerType>();
        ptr && keep_pointee_sugar &&
        keep_pointee_sugar(ptr->getPointeeType())) {
      auto node = std::make_unique<PointerNode>(
          Kind::kPointer, fromType(ptr->getPointeeType(), true));
      node->type = canonical.getUnqualifiedType();
      return qualify(std::move(node), canonical);
    }
  }
  return fromCanonical(type.getCanonicalType());
}

NodePtr Builder::fromCanonical(clang::QualType canonical) {
  NodePtr node;
  const auto *type = canonical.getTypePtr();
  bool quals_on_element = false;
  if (const auto *builtin = llvm::dyn_cast<clang::BuiltinType>(type)) {
    clang::PrintingPolicy policy(ctx_.getLangOpts());
    policy.Bool = true;
    node =
        std::make_unique<Node>(Kind::kBuiltin, builtin->getName(policy).str());
  } else if (const auto *ptr = llvm::dyn_cast<clang::PointerType>(type)) {
    node = std::make_unique<PointerNode>(
        Kind::kPointer, fromType(ptr->getPointeeType(), false));
  } else if (const auto *ref =
                 llvm::dyn_cast<clang::LValueReferenceType>(type)) {
    node = std::make_unique<PointerNode>(
        Kind::kLRef, fromType(ref->getPointeeType(), false));
  } else if (const auto *ref =
                 llvm::dyn_cast<clang::RValueReferenceType>(type)) {
    node = std::make_unique<PointerNode>(
        Kind::kRRef, fromType(ref->getPointeeType(), false));
  } else if (const auto *array = ctx_.getAsConstantArrayType(canonical)) {
    auto array_node =
        std::make_unique<ArrayNode>(fromType(array->getElementType(), false));
    array_node->size = std::make_unique<Node>(
        Kind::kValue, llvm::toString(array->getSize(), 10, false));
    node = std::move(array_node);
    quals_on_element = true;
  } else if (const auto *array = ctx_.getAsIncompleteArrayType(canonical)) {
    node =
        std::make_unique<ArrayNode>(fromType(array->getElementType(), false));
    quals_on_element = true;
  } else if (const auto *record = type->getAsRecordDecl()) {
    node = fromRecord(record);
  } else if (const auto *enum_type = llvm::dyn_cast<clang::EnumType>(type)) {
    node = std::make_unique<Node>(Kind::kEnum, tagName(enum_type->getDecl()));
  } else if (const auto *proto =
                 llvm::dyn_cast<clang::FunctionProtoType>(type)) {
    auto fn = std::make_unique<FunctionNode>(Kind::kFunctionType);
    fn->variadic = proto->isVariadic();
    fn->return_type = fromType(proto->getReturnType(), false);
    for (auto param : proto->getParamTypes()) {
      fn->params.push_back(fromType(param, false));
    }
    node = std::move(fn);
  } else {
    node = std::make_unique<Node>(Kind::kOpaque,
                                  canonical.getUnqualifiedType().getAsString());
  }
  if (quals_on_element) {
    node->type = canonical;
    return node;
  }
  node->type = canonical.getUnqualifiedType();
  return qualify(std::move(node), canonical);
}

NodePtr Builder::fromRecord(const clang::RecordDecl *decl) {
  if (stand_in) {
    if (auto param = stand_in(decl)) {
      return std::make_unique<ParamNode>(*param);
    }
  }
  if (const auto *cxx = llvm::dyn_cast<clang::CXXRecordDecl>(decl);
      cxx && cxx->isLambda()) {
    return std::make_unique<Node>(Kind::kOpaque, "lambda");
  }
  auto node = std::make_unique<RecordNode>(tagName(decl));
  node->class_ = classOf(decl);
  if (const auto *spec =
          llvm::dyn_cast<clang::ClassTemplateSpecializationDecl>(decl)) {
    for (const auto &arg : spec->getTemplateArgs().asArray()) {
      node->args.push_back(fromTemplateArg(arg));
    }
  }
  return node;
}

NodePtr Builder::fromTemplateArg(const clang::TemplateArgument &arg) {
  switch (arg.getKind()) {
  case clang::TemplateArgument::Type:
    return fromType(arg.getAsType(), false);
  case clang::TemplateArgument::Integral: {
    auto node = std::make_unique<Node>(Kind::kValue,
                                       llvm::toString(arg.getAsIntegral(), 10));
    node->type = arg.getIntegralType();
    return node;
  }
  default: {
    std::string spelling;
    llvm::raw_string_ostream os(spelling);
    arg.print(clang::PrintingPolicy(ctx_.getLangOpts()), os,
              /*IncludeType=*/true);
    return std::make_unique<Node>(Kind::kOpaque, spelling);
  }
  }
}

NodePtr Builder::classOf(const clang::Decl *decl) {
  if (const auto *record =
          llvm::dyn_cast<clang::RecordDecl>(decl->getDeclContext())) {
    auto node = fromRecord(record);
    node->type = ctx_.getCanonicalTagType(record);
    return node;
  }
  return nullptr;
}

NodePtr Builder::FromDecl(const clang::NamedDecl *decl) {
  if (const auto *tmpl = llvm::dyn_cast<clang::FunctionTemplateDecl>(decl)) {
    decl = tmpl->getTemplatedDecl();
  }
  const auto *func = llvm::dyn_cast<clang::FunctionDecl>(decl);
  if (!func) {
    auto node = std::make_unique<DeclNode>(QualifiedName(decl));
    node->class_ = classOf(decl);
    return node;
  }

  auto node =
      std::make_unique<FunctionNode>(Kind::kFunction, QualifiedName(func));
  node->variadic = func->isVariadic();
  node->class_ = classOf(func);
  node->return_type = fromType(func->getReturnType(), false);
  bool has_pack = HasFunctionParameterPack(func);
  unsigned num_params = func->getNumParams();
  if (has_pack) {
    const auto *primary = func->getPrimaryTemplate();
    num_params =
        (primary ? primary->getTemplatedDecl() : func)->getNumParams() - 1;
  }
  for (unsigned i = 0; i < num_params; ++i) {
    node->params.push_back(fromType(func->getParamDecl(i)->getType(), false));
  }
  if (has_pack) {
    node->params.push_back(std::make_unique<Node>(Kind::kPackParams));
  }
  if (const auto *method = llvm::dyn_cast<clang::CXXMethodDecl>(func)) {
    node->is_const = method->isConst();
    node->is_volatile = method->isVolatile();
    switch (method->getRefQualifier()) {
    case clang::RQ_LValue:
      node->ref = "&";
      break;
    case clang::RQ_RValue:
      node->ref = "&&";
      break;
    default:
      break;
    }
  }
  return node;
}

NodePtr Builder::FromExpr(const clang::Expr *expr) {
  expr = expr->IgnoreParenImpCasts();

  if (llvm::isa<clang::IntegerLiteral>(expr) &&
      expr->getBeginLoc().isMacroID()) {
    auto name = clang::Lexer::getImmediateMacroName(
        expr->getBeginLoc(), ctx_.getSourceManager(), ctx_.getLangOpts());
    if (!name.empty()) {
      return std::make_unique<Node>(Kind::kMacro, name.str());
    }
  }

  if (const auto *call = llvm::dyn_cast<clang::CallExpr>(expr)) {
    if (const auto *callee = call->getDirectCallee()) {
      return FromDecl(callee);
    }
  }

  if (const auto *ctor = llvm::dyn_cast<clang::CXXConstructExpr>(expr)) {
    assert(ctor->getConstructor() &&
           "expr is a CXXConstructExpr but could not get constructor");
    return FromDecl(ctor->getConstructor());
  }

  if (const auto *member = llvm::dyn_cast<clang::MemberExpr>(expr)) {
    const auto *decl = member->getMemberDecl();
    if (llvm::isa<clang::CXXMethodDecl>(decl)) {
      return FromDecl(decl);
    }
    auto arrow = [&](clang::QualType object) {
      return std::make_unique<ArrowNode>(FromType(object), FromDecl(decl));
    };
    if (member->isArrow()) {
      const auto *base = member->getBase()->IgnoreParenImpCasts();
      if (const auto *op = llvm::dyn_cast<clang::CXXOperatorCallExpr>(base);
          op && op->getOperator() == clang::OO_Arrow) {
        return arrow(op->getArg(0)->IgnoreImpCasts()->getType());
      }
    } else if (auto for_range = GetParentForRange(ctx_, member)) {
      const auto *range =
          for_range->getRangeInit()->getType()->getAsCXXRecordDecl();
      if (range && llvm::isa<clang::ClassTemplateSpecializationDecl>(range) &&
          QualifiedName(range) == "std::map") {
        auto iter_type = GetForRangeIteratorType(for_range);
        if (!iter_type.isNull()) {
          return arrow(iter_type);
        }
      }
    }
    return FromDecl(decl);
  }

  if (const auto *ref = llvm::dyn_cast<clang::DeclRefExpr>(expr)) {
    return FromDecl(ref->getDecl());
  }

  if (const auto *uop = llvm::dyn_cast<clang::UnaryOperator>(expr)) {
    auto sub = FromExpr(uop->getSubExpr());
    if (!sub) {
      return nullptr;
    }
    return std::make_unique<UnaryNode>(
        (uop->isPostfix() ? "post" : "") +
            clang::UnaryOperator::getOpcodeStr(uop->getOpcode()).str(),
        std::move(sub));
  }

  return nullptr;
}

} // namespace cpp2rust::IrSrc
