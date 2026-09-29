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

Node ParseNodeJSON(const llvm::json::Value &value) {
  const auto *obj = value.getAsObject();
  assert(obj && "IR node must be an object");
  Node node;
  node.kind = kindFromName(*obj->getString("kind"));
  if (auto name = obj->getString("name")) {
    node.name = name->str();
  }
  if (auto param = obj->getInteger("param")) {
    node.param = *param;
  }
  node.is_const = obj->getBoolean("is_const").value_or(false);
  node.is_volatile = obj->getBoolean("is_volatile").value_or(false);
  node.variadic = obj->getBoolean("variadic").value_or(false);
  if (auto ref = obj->getString("ref")) {
    node.ref = ref->str();
  }
  for (const auto &[key, field] : kNodeFields) {
    if (const auto *child = obj->get(key)) {
      node.*field = Share(ParseNodeJSON(*child));
    }
  }
  for (const auto &[key, field] : kNodeLists) {
    if (const auto *list = obj->getArray(key)) {
      for (const auto &child : *list) {
        (node.*field).push_back(ParseNodeJSON(child));
      }
    }
  }
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
  rule.num_params = NumParams(rule.ir);
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
  rule.num_params = NumParams(rule.ir);
  return rule;
}

Node qualify(Node node, clang::QualType type) {
  if (type.isVolatileQualified()) {
    Node wrapper = Make(Kind::kVolatile);
    wrapper.type = node.type.withVolatile();
    wrapper.operand = Share(std::move(node));
    node = std::move(wrapper);
  }
  if (type.isConstQualified()) {
    Node wrapper = Make(Kind::kConst);
    wrapper.type = node.type.withConst();
    wrapper.operand = Share(std::move(node));
    node = std::move(wrapper);
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
         zipChildren(*this, other,
                     [](const Node &a, const Node &b) { return a == b; });
}

bool Node::shallowEquals(const Node &other) const {
  return kind == other.kind && name == other.name && param == other.param &&
         is_const == other.is_const && is_volatile == other.is_volatile &&
         variadic == other.variadic && ref == other.ref;
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
    return "unary" + name + ":" + operand->indexKey();
  case Kind::kArrow:
    return member->indexKey();
  case Kind::kConst:
  case Kind::kVolatile:
    return operand->indexKey();
  case Kind::kPointer:
    return "*" + pointee->indexKey();
  case Kind::kLRef:
    return "&" + pointee->indexKey();
  case Kind::kRRef:
    return "&&" + pointee->indexKey();
  case Kind::kArray:
    return "[]" + element->indexKey();
  default:
    return "";
  }
}

unsigned Node::specificity() const {
  unsigned n = kind == Kind::kParam ? 0 : 1;
  zipChildren(*this, *this, [&](const Node &child, const Node &) {
    n += child.specificity();
    return true;
  });
  return n;
}

bool Node::hasParam(unsigned n) const {
  if (kind == Kind::kParam) {
    return param == n;
  }
  return !zipChildren(*this, *this, [&](const Node &child, const Node &) {
    return !child.hasParam(n);
  });
}

void Node::forEachParam(const std::function<void(unsigned)> &fn) const {
  if (kind == Kind::kParam) {
    fn(param);
  }
  zipChildren(*this, *this, [&](const Node &child, const Node &) {
    child.forEachParam(fn);
    return true;
  });
}

std::string Node::str() const {
  std::string out;
  if (is_const) {
    out += "const ";
  }
  if (is_volatile) {
    out += "volatile ";
  }
  if (kind == Kind::kParam) {
    return out + "T" + std::to_string(param);
  }
  if (kind == Kind::kConst || kind == Kind::kVolatile) {
    return out + kindName(kind) + " " + operand->str();
  }
  out += kindName(kind);
  if (!name.empty()) {
    out += " " + name;
  }
  std::vector<std::string> fields;
  for (const auto &[key, field] : kNodeFields) {
    if (this->*field) {
      fields.push_back(std::string(key) + ": " + (this->*field)->str());
    }
  }
  for (const auto &[key, field] : kNodeLists) {
    if (!(this->*field).empty()) {
      std::string list;
      for (const auto &child : this->*field) {
        list += (list.empty() ? "" : ", ") + child.str();
      }
      fields.push_back(std::string(key) + ": [" + list + "]");
    }
  }
  if (!fields.empty()) {
    out += "(";
    for (size_t i = 0; i < fields.size(); ++i) {
      out += (i ? ", " : "") + fields[i];
    }
    out += ")";
  }
  return out;
}

Node Make(Kind kind, std::string name) {
  Node node;
  node.kind = kind;
  node.name = std::move(name);
  return node;
}

std::shared_ptr<Node> Share(Node node) {
  return std::make_shared<Node>(std::move(node));
}

llvm::json::Value ToJSON(const Node &node) {
  llvm::json::Object obj{{"kind", kindName(node.kind)}};
  if (!node.name.empty()) {
    obj["name"] = node.name;
  }
  if (node.kind == Kind::kParam) {
    obj["param"] = node.param;
  }
  if (node.is_const) {
    obj["is_const"] = true;
  }
  if (node.is_volatile) {
    obj["is_volatile"] = true;
  }
  if (node.variadic) {
    obj["variadic"] = true;
  }
  if (!node.ref.empty()) {
    obj["ref"] = node.ref;
  }
  for (const auto &[key, field] : kNodeFields) {
    if (node.*field) {
      obj[key] = ToJSON(*(node.*field));
    }
  }
  for (const auto &[key, field] : kNodeLists) {
    if (!(node.*field).empty()) {
      llvm::json::Array list;
      for (const auto &child : node.*field) {
        list.push_back(ToJSON(child));
      }
      obj[key] = std::move(list);
    }
  }
  return obj;
}

void ExprRule::dump() const {
  log() << "Matching: " << ir.str() << '\n';
  if (init_type.valid()) {
    log() << "  init type: depth " << init_type.depth << ", index "
          << init_type.index << '\n';
  }
}

void TypeRule::dump() const { log() << "name: " << ir.str() << '\n'; }

Rules Load(const std::filesystem::path &dir) {
  Rules rules;
  Ir::LoadJSON(rules, dir / "ir_src.json", true, ParseExprRuleJSON,
               ParseTypeRuleJSON);
  return rules;
}

Node Builder::FromType(clang::QualType type) { return fromType(type, true); }

Node Builder::fromType(clang::QualType type, bool top) {
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
      Node node = Make(Kind::kTypedef, name->getName().str());
      node.type = canonical.getUnqualifiedType();
      return qualify(std::move(node), canonical);
    }
    if (const auto *predef = type->getAs<clang::PredefinedSugarType>()) {
      Node node =
          Make(Kind::kTypedef, predef->getIdentifier()->getName().str());
      node.type = canonical.getUnqualifiedType();
      return qualify(std::move(node), canonical);
    }
    if (const auto *ptr = type->getAs<clang::PointerType>();
        ptr && keep_pointee_sugar &&
        keep_pointee_sugar(ptr->getPointeeType())) {
      Node node = Make(Kind::kPointer);
      node.pointee = Share(fromType(ptr->getPointeeType(), true));
      node.type = canonical.getUnqualifiedType();
      return qualify(std::move(node), canonical);
    }
  }
  return fromCanonical(type.getCanonicalType());
}

Node Builder::fromCanonical(clang::QualType canonical) {
  Node node;
  const auto *type = canonical.getTypePtr();
  bool quals_on_element = false;
  if (const auto *builtin = llvm::dyn_cast<clang::BuiltinType>(type)) {
    clang::PrintingPolicy policy(ctx_.getLangOpts());
    policy.Bool = true;
    node = Make(Kind::kBuiltin, builtin->getName(policy).str());
  } else if (const auto *ptr = llvm::dyn_cast<clang::PointerType>(type)) {
    node = Make(Kind::kPointer);
    node.pointee = Share(fromType(ptr->getPointeeType(), false));
  } else if (const auto *ref =
                 llvm::dyn_cast<clang::LValueReferenceType>(type)) {
    node = Make(Kind::kLRef);
    node.pointee = Share(fromType(ref->getPointeeType(), false));
  } else if (const auto *ref =
                 llvm::dyn_cast<clang::RValueReferenceType>(type)) {
    node = Make(Kind::kRRef);
    node.pointee = Share(fromType(ref->getPointeeType(), false));
  } else if (const auto *array = ctx_.getAsConstantArrayType(canonical)) {
    node = Make(Kind::kArray);
    node.element = Share(fromType(array->getElementType(), false));
    node.size =
        Share(Make(Kind::kValue, llvm::toString(array->getSize(), 10, false)));
    quals_on_element = true;
  } else if (const auto *array = ctx_.getAsIncompleteArrayType(canonical)) {
    node = Make(Kind::kArray);
    node.element = Share(fromType(array->getElementType(), false));
    quals_on_element = true;
  } else if (const auto *record = type->getAsRecordDecl()) {
    node = fromRecord(record);
  } else if (const auto *enum_type = llvm::dyn_cast<clang::EnumType>(type)) {
    node = Make(Kind::kEnum, tagName(enum_type->getDecl()));
  } else if (const auto *proto =
                 llvm::dyn_cast<clang::FunctionProtoType>(type)) {
    node = Make(Kind::kFunctionType);
    node.variadic = proto->isVariadic();
    node.return_type = Share(fromType(proto->getReturnType(), false));
    for (auto param : proto->getParamTypes()) {
      node.params.push_back(fromType(param, false));
    }
  } else {
    node = Make(Kind::kOpaque, canonical.getUnqualifiedType().getAsString());
  }
  if (quals_on_element) {
    node.type = canonical;
    return node;
  }
  node.type = canonical.getUnqualifiedType();
  return qualify(std::move(node), canonical);
}

Node Builder::fromRecord(const clang::RecordDecl *decl) {
  if (stand_in) {
    if (auto param = stand_in(decl)) {
      Node node = Make(Kind::kParam);
      node.param = *param;
      return node;
    }
  }
  if (const auto *cxx = llvm::dyn_cast<clang::CXXRecordDecl>(decl);
      cxx && cxx->isLambda()) {
    return Make(Kind::kOpaque, "lambda");
  }
  Node node = Make(Kind::kRecord, tagName(decl));
  node.class_ = classOf(decl);
  if (const auto *spec =
          llvm::dyn_cast<clang::ClassTemplateSpecializationDecl>(decl)) {
    for (const auto &arg : spec->getTemplateArgs().asArray()) {
      node.args.push_back(fromTemplateArg(arg));
    }
  }
  return node;
}

Node Builder::fromTemplateArg(const clang::TemplateArgument &arg) {
  switch (arg.getKind()) {
  case clang::TemplateArgument::Type:
    return fromType(arg.getAsType(), false);
  case clang::TemplateArgument::Integral: {
    Node node = Make(Kind::kValue, llvm::toString(arg.getAsIntegral(), 10));
    node.type = arg.getIntegralType();
    return node;
  }
  default: {
    std::string spelling;
    llvm::raw_string_ostream os(spelling);
    arg.print(clang::PrintingPolicy(ctx_.getLangOpts()), os,
              /*IncludeType=*/true);
    return Make(Kind::kOpaque, spelling);
  }
  }
}

std::shared_ptr<Node> Builder::classOf(const clang::Decl *decl) {
  if (const auto *record =
          llvm::dyn_cast<clang::RecordDecl>(decl->getDeclContext())) {
    Node node = fromRecord(record);
    node.type = ctx_.getCanonicalTagType(record);
    return Share(std::move(node));
  }
  return nullptr;
}

Node Builder::FromDecl(const clang::NamedDecl *decl) {
  if (const auto *tmpl = llvm::dyn_cast<clang::FunctionTemplateDecl>(decl)) {
    decl = tmpl->getTemplatedDecl();
  }
  const auto *func = llvm::dyn_cast<clang::FunctionDecl>(decl);
  if (!func) {
    Node node = Make(Kind::kDecl, QualifiedName(decl));
    node.class_ = classOf(decl);
    return node;
  }

  Node node = Make(Kind::kFunction, QualifiedName(func));
  node.variadic = func->isVariadic();
  node.class_ = classOf(func);
  node.return_type = Share(fromType(func->getReturnType(), false));
  bool has_pack = HasFunctionParameterPack(func);
  unsigned num_params = func->getNumParams();
  if (has_pack) {
    const auto *primary = func->getPrimaryTemplate();
    num_params =
        (primary ? primary->getTemplatedDecl() : func)->getNumParams() - 1;
  }
  for (unsigned i = 0; i < num_params; ++i) {
    node.params.push_back(fromType(func->getParamDecl(i)->getType(), false));
  }
  if (has_pack) {
    node.params.push_back(Make(Kind::kPackParams));
  }
  if (const auto *method = llvm::dyn_cast<clang::CXXMethodDecl>(func)) {
    node.is_const = method->isConst();
    node.is_volatile = method->isVolatile();
    switch (method->getRefQualifier()) {
    case clang::RQ_LValue:
      node.ref = "&";
      break;
    case clang::RQ_RValue:
      node.ref = "&&";
      break;
    default:
      break;
    }
  }
  return node;
}

std::optional<Node> Builder::FromExpr(const clang::Expr *expr) {
  expr = expr->IgnoreParenImpCasts();

  if (llvm::isa<clang::IntegerLiteral>(expr) &&
      expr->getBeginLoc().isMacroID()) {
    auto name = clang::Lexer::getImmediateMacroName(
        expr->getBeginLoc(), ctx_.getSourceManager(), ctx_.getLangOpts());
    if (!name.empty()) {
      return Make(Kind::kMacro, name.str());
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
      Node node = Make(Kind::kArrow);
      node.object = Share(FromType(object));
      node.member = Share(FromDecl(decl));
      return node;
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
      return std::nullopt;
    }
    Node node =
        Make(Kind::kUnary,
             (uop->isPostfix() ? "post" : "") +
                 clang::UnaryOperator::getOpcodeStr(uop->getOpcode()).str());
    node.operand = Share(std::move(*sub));
    return node;
  }

  return std::nullopt;
}

} // namespace cpp2rust::IrSrc
