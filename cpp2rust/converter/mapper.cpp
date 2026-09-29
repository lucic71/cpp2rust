// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/mapper.h"

#include <clang/AST/DeclCXX.h>
#include <clang/AST/DeclTemplate.h>
#include <clang/AST/ExprCXX.h>
#include <clang/AST/PrettyPrinter.h>
#include <clang/Basic/OperatorKinds.h>
#include <clang/Basic/SourceManager.h>
#include <clang/Lex/Lexer.h>
#include <llvm/Support/ThreadPool.h>

#include <cctype>
#include <cstdlib>
#include <format>
#include <optional>
#include <regex>
#include <unordered_map>
#include <utility>
#include <vector>

#include "converter/converter_lib.h"
#include "converter/rules/ir_src.h"
#include "converter/rules/ir_tgt.h"

namespace cpp2rust::Mapper {

namespace {

clang::ASTContext *ctx_ = nullptr;
Model model_ = Model::kUnsafe;
bool translation_rules_loaded_ = false;

struct ExprRule {
  IrSrc::ExprRule src;
  IrTgt::ExprRule tgt;
};

struct TypeRule {
  IrSrc::TypeRule src;
  IrTgt::TypeRule tgt;
};

std::unordered_multimap<std::string, ExprRule> exprs_;
std::unordered_multimap<std::string, TypeRule> types_;

clang::PrintingPolicy getPrintPolicy() {
  assert(ctx_);
  clang::PrintingPolicy policy(ctx_->getLangOpts());
  policy.Bool = true;
  policy.SuppressTagKeyword = true;
  policy.SuppressScope = false;
  policy.FullyQualifiedName = true;
  policy.SuppressUnwrittenScope = true;
  policy.UsePreferredNames = true;
  return policy;
}

constexpr const char kPackMarker[] = "&&...";

using Node = IrSrc::Node;
using Kind = Node::Kind;
using Bindings = std::vector<std::optional<Node>>;

std::shared_ptr<Node> share(Node node) {
  return std::make_shared<Node>(std::move(node));
}

Node make(Kind kind, std::string name = {}) {
  Node node;
  node.kind = kind;
  node.name = std::move(name);
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
  if (!tag->getIdentifier() || tag->getDeclContext()->isFunctionOrMethod()) {
    return ToString(tag->getASTContext().getCanonicalTagType(tag));
  }
  return QualifiedName(tag);
}

std::string IndexKey(const Node &node) {
  switch (node.kind) {
  case Kind::kFunction:
  case Kind::kDecl:
  case Kind::kRecord:
  case Kind::kEnum:
  case Kind::kTypedef:
  case Kind::kBuiltin:
    return node.name;
  case Kind::kMacro:
    return "macro:" + node.name;
  case Kind::kUnary:
    return "unary" + node.name + ":" + IndexKey(*node.operand);
  case Kind::kArrow:
    return IndexKey(*node.member);
  case Kind::kPointer:
    return "*" + IndexKey(*node.pointee);
  case Kind::kLRef:
    return "&" + IndexKey(*node.pointee);
  case Kind::kRRef:
    return "&&" + IndexKey(*node.pointee);
  case Kind::kArray:
    return "[]" + IndexKey(*node.element);
  default:
    return "";
  }
}

bool Match(const Node &rule, const Node &use, Bindings &bindings) {
  if (rule.kind == Kind::kParam) {
    if ((rule.is_const && !use.is_const) ||
        (rule.is_volatile && !use.is_volatile)) {
      return false;
    }
    Node bound = use;
    if (rule.is_const) {
      bound.is_const = false;
      bound.type.removeLocalConst();
    }
    if (rule.is_volatile) {
      bound.is_volatile = false;
      bound.type.removeLocalVolatile();
    }
    if (bindings.size() <= rule.param) {
      bindings.resize(rule.param + 1);
    }
    auto &slot = bindings[rule.param];
    if (!slot) {
      slot = std::move(bound);
      return true;
    }
    return *slot == bound;
  }
  if (rule.kind != use.kind || rule.name != use.name ||
      rule.is_const != use.is_const || rule.is_volatile != use.is_volatile ||
      rule.variadic != use.variadic || rule.ref != use.ref) {
    return false;
  }
  return Node::zipChildren(rule, use, [&](const Node &r, const Node &u) {
    return Match(r, u, bindings);
  });
}

void AddTypeRule(clang::QualType type, IrTgt::TypeRule &&rule) {
  auto src = IrSrcBuilder(*ctx_).FromType(type);
  auto key = IndexKey(src);
  auto [begin, end] = types_.equal_range(key);
  for (auto it = begin; it != end; ++it) {
    if (it->second.src.ir == src) {
      return;
    }
  }
  types_.emplace(std::move(key), TypeRule{{std::move(src)}, std::move(rule)});
}

std::string instantiateTgt(const Bindings &bindings,
                           const std::string &tgt_template) {
  std::string instantiated_template = tgt_template;
  std::string::size_type pos = 0;
  while ((pos = instantiated_template.find('T', pos)) != std::string::npos) {
    if (pos + 1 >= instantiated_template.size()) {
      break;
    }
    if (!std::isdigit(instantiated_template[pos + 1])) {
      ++pos;
      continue;
    }
    unsigned n = instantiated_template[pos + 1] - '0';
    assert(n < bindings.size() && bindings[n] &&
           "target uses a template parameter the use site does not bind");
    const auto &bound = *bindings[n];
    std::string repl;
    if (bound.kind == Kind::kValue) {
      repl = bound.name;
    } else {
      assert(!bound.type.isNull() && "template parameter bound to a non-type");
      repl = Map(bound.type);
      if (repl.empty()) {
        llvm::errs() << "cpp_type: " << ToString(bound.type) << '\n';
        assert(0 && "Type is not present in types_");
      }
    }
    instantiated_template.replace(pos, 2, repl);
    pos += repl.length();
  }
  return instantiated_template;
}

template <typename T>
std::pair<T *, Bindings> search(std::unordered_multimap<std::string, T> &map,
                                const Node &use, const std::string &key) {
  auto [it, end] = map.equal_range(key);
  T *rule = nullptr;
  Bindings bindings;
  unsigned specificity = 0;
  for (; it != end; ++it) {
    Bindings these;
    const auto &ir = it->second.src.ir;
    if (!Match(ir, use, these)) {
      continue;
    }
    if (!rule || ir.specificity() > specificity) {
      rule = &it->second;
      bindings = std::move(these);
      specificity = ir.specificity();
    }
  }
  return {rule, std::move(bindings)};
}

std::pair<ExprRule *, Bindings> search(const clang::Expr *expr) {
  if (RefersToUserDefinedDecl(expr)) {
    return {};
  }
  auto use = IrSrcBuilder(*ctx_).FromExpr(expr);
  if (!use) {
    return {};
  }
  auto res = search(exprs_, *use, IndexKey(*use));
  log() << "search expr " << use->str() << ", result:\n";
  if (res.first) {
    res.first->src.dump();
    res.first->tgt.dump();
  } else {
    log() << "None\n";
  }
  return res;
}

Node typeIR(clang::QualType qual_type, bool sugared) {
  IrSrcBuilder builder(*ctx_);
  if (sugared) {
    builder.keep_builtin_typedef = true;
    builder.keep_pointee_sugar = [](clang::QualType pointee) {
      auto canonical = pointee.getCanonicalType().getDesugaredType(*ctx_);
      bool builtin_alias = canonical->isBuiltinType() &&
                           (pointee->getAs<clang::TypedefType>() ||
                            pointee->getAs<clang::PredefinedSugarType>());
      return builtin_alias || Map(pointee) != Map(canonical);
    };
  }
  auto node = builder.FromType(qual_type);
  if (node.kind != Kind::kArray) {
    node.is_const = false;
    node.is_volatile = false;
  }
  return node;
}

std::pair<IrTgt::TypeRule *, Bindings> search(clang::QualType qual_type) {
  for (bool sugared : {true, false}) {
    auto use = typeIR(qual_type, sugared);
    auto key = IndexKey(use);
    auto [rule, bindings] = search(types_, use, key);
    if (!rule && !key.empty()) {
      std::tie(rule, bindings) = search(types_, use, "");
    }
    if (rule) {
      log() << "search type " << use.str()
            << ", result: " << rule->tgt.type_info.type << '\n';
      return {&rule->tgt, std::move(bindings)};
    }
  }
  log() << "search type " << ToString(qual_type) << ", result: None\n";
  return {};
}

void validate(const std::string &name, const ExprRule &rule) {
  const auto &[src, tgt] = rule;
  if (tgt.usesInit() && !src.init_type.valid()) {
    llvm::errs() << name << '\n';
    tgt.dump();
    llvm::report_fatal_error(
        "Expr rule uses init but its src pack is not declared as Init<T, "
        "Args>");
  }

  bool has_generic[Ir::kMaxGenerics] = {false};
  src.ir.forEachParam([&](unsigned n) {
    if (n >= 1 && n <= Ir::kMaxGenerics) {
      has_generic[n - 1] = true;
    }
  });

  for (size_t i = 0, e = tgt.generics.size(); i < e; ++i) {
    if (!has_generic[i]) {
      llvm::errs() << name << '\n';
      tgt.dump();
      llvm::errs() << "generic T" << (i + 1)
                   << " declared but missing from src: " << src.ir.str()
                   << '\n';
      llvm::report_fatal_error("Absent generic from src");
    }
  }
}

template <typename Src>
Src takeSrc(std::unordered_map<std::string, Src> &src, const std::string &name,
            const std::filesystem::path &dir) {
  auto it = src.find(name);
  if (it == src.end()) {
    llvm::errs() << "ERROR: " << dir.string() << ": rule " << name
                 << " has no entry in ir_src.json\n";
    std::exit(EXIT_FAILURE);
  }
  auto rule = std::move(it->second);
  src.erase(it);
  return rule;
}

void addRulesFromDirectory(const std::filesystem::path &dir, Model model) {
  namespace fs = std::filesystem;
  for (const auto &entry : fs::directory_iterator(dir)) {
    const auto &path = entry.path();
    assert(fs::exists(path / "ir_src.json") &&
           (fs::exists(path / "ir_unsafe.json") ||
            fs::exists(path / "ir_refcount.json")));
    auto src = IrSrc::Load(path);
    auto tgt = IrTgt::Load(path, model);
    if (tgt.exprs.empty() && tgt.types.empty()) {
      log() << "No rules found in " << path << '\n';
      continue;
    }
    for (auto &[name, rule] : tgt.exprs) {
      ExprRule paired{takeSrc(src.exprs, name, path), std::move(rule)};
      validate(name, paired);
      auto key = IndexKey(paired.src.ir);
      exprs_.emplace(std::move(key), std::move(paired));
    }
    for (auto &[name, rule] : tgt.types) {
      TypeRule paired{takeSrc(src.types, name, path), std::move(rule)};
      auto key = IndexKey(paired.src.ir);
      auto [begin, end] = types_.equal_range(key);
      for (auto it = begin; it != end; ++it) {
        if (it->second.src.ir == paired.src.ir) {
          llvm::errs() << "ERROR: duplicate type rule for C++ type '"
                       << paired.src.ir.str() << "': maps to both '"
                       << it->second.tgt.type_info.type << "' and '"
                       << paired.tgt.type_info.type << "'\n";
          std::exit(EXIT_FAILURE);
        }
      }
      types_.emplace(std::move(key), std::move(paired));
    }
    if (!src.exprs.empty() || !src.types.empty()) {
      llvm::errs() << "ERROR: " << path.string()
                   << ": ir_src.json has rules without a target rule\n";
      std::exit(EXIT_FAILURE);
    }
  }
}

clang::QualType normalizeQualType(clang::QualType qual_type) {
  assert(ctx_);

  bool isLRef = qual_type->isLValueReferenceType();
  bool isRRef = qual_type->isRValueReferenceType();
  qual_type = qual_type.getNonReferenceType();

  clang::Qualifiers qualifiers = qual_type.getQualifiers();

  while (true) {
    if (const auto *attributed =
            llvm::dyn_cast<clang::AttributedType>(qual_type)) {
      qual_type = attributed->getModifiedType();
      continue;
    }
    if (const auto *dcltype = llvm::dyn_cast<clang::DecltypeType>(qual_type)) {
      qual_type = dcltype->getUnderlyingType();
      continue;
    }
    break;
  }

  if (llvm::isa<clang::InjectedClassNameType>(qual_type)) {
    qual_type = qual_type.getCanonicalType();
  }

  qual_type = qual_type.withFastQualifiers(qualifiers.getFastQualifiers());
  if (qualifiers.hasNonFastQualifiers()) {
    qual_type = ctx_->getQualifiedType(qual_type, qualifiers);
  }

  if (isLRef) {
    qual_type = ctx_->getLValueReferenceType(qual_type);
  }

  if (isRRef) {
    qual_type = ctx_->getRValueReferenceType(qual_type);
  }

  return qual_type.getCanonicalType().getUnqualifiedType().getDesugaredType(
      *ctx_);
}

std::string normalizeTranslationRule(std::string rule) {
  // Detach pointer from double reference. Useful for matching translation
  // rules.
  ReplaceAll(rule, "*&&", "* &&");

  static const std::array<std::pair<std::regex, std::string>, 1>
      normalization_rules{{
          // Ignore constant template parameters, i.e. replace them with _.
          {std::regex(R"(\b\d+\b)"), "_"},
      }};

  for (const auto &r : normalization_rules) {
    rule = std::regex_replace(rule, r.first, r.second);
  }

  return rule;
}

} // namespace

Node IrSrcBuilder::FromType(clang::QualType type) {
  return fromType(type, true);
}

Node IrSrcBuilder::fromType(clang::QualType type, bool top) {
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
      Node node = make(Kind::kTypedef, name->getName().str());
      node.is_const = canonical.isConstQualified();
      node.is_volatile = canonical.isVolatileQualified();
      node.type = canonical;
      return node;
    }
    if (const auto *predef = type->getAs<clang::PredefinedSugarType>()) {
      Node node =
          make(Kind::kTypedef, predef->getIdentifier()->getName().str());
      node.is_const = canonical.isConstQualified();
      node.is_volatile = canonical.isVolatileQualified();
      node.type = canonical;
      return node;
    }
    if (const auto *ptr = type->getAs<clang::PointerType>();
        ptr && keep_pointee_sugar &&
        keep_pointee_sugar(ptr->getPointeeType())) {
      Node node = make(Kind::kPointer);
      node.pointee = share(fromType(ptr->getPointeeType(), true));
      node.is_const = canonical.isConstQualified();
      node.is_volatile = canonical.isVolatileQualified();
      node.type = canonical;
      return node;
    }
  }
  return fromCanonical(type.getCanonicalType());
}

Node IrSrcBuilder::fromCanonical(clang::QualType canonical) {
  Node node;
  const auto *type = canonical.getTypePtr();
  bool quals_on_element = false;
  if (const auto *builtin = llvm::dyn_cast<clang::BuiltinType>(type)) {
    clang::PrintingPolicy policy(ctx_.getLangOpts());
    policy.Bool = true;
    node = make(Kind::kBuiltin, builtin->getName(policy).str());
  } else if (const auto *ptr = llvm::dyn_cast<clang::PointerType>(type)) {
    node = make(Kind::kPointer);
    node.pointee = share(fromType(ptr->getPointeeType(), false));
  } else if (const auto *ref =
                 llvm::dyn_cast<clang::LValueReferenceType>(type)) {
    node = make(Kind::kLRef);
    node.pointee = share(fromType(ref->getPointeeType(), false));
  } else if (const auto *ref =
                 llvm::dyn_cast<clang::RValueReferenceType>(type)) {
    node = make(Kind::kRRef);
    node.pointee = share(fromType(ref->getPointeeType(), false));
  } else if (const auto *array = ctx_.getAsConstantArrayType(canonical)) {
    node = make(Kind::kArray);
    node.element = share(fromType(array->getElementType(), false));
    node.size =
        share(make(Kind::kValue, llvm::toString(array->getSize(), 10, false)));
    quals_on_element = true;
  } else if (const auto *array = ctx_.getAsIncompleteArrayType(canonical)) {
    node = make(Kind::kArray);
    node.element = share(fromType(array->getElementType(), false));
    quals_on_element = true;
  } else if (const auto *record = type->getAsRecordDecl()) {
    node = fromRecord(record);
  } else if (const auto *enum_type = llvm::dyn_cast<clang::EnumType>(type)) {
    node = make(Kind::kEnum, tagName(enum_type->getDecl()));
  } else if (const auto *proto =
                 llvm::dyn_cast<clang::FunctionProtoType>(type)) {
    node = make(Kind::kFunctionType);
    node.variadic = proto->isVariadic();
    node.return_type = share(fromType(proto->getReturnType(), false));
    for (auto param : proto->getParamTypes()) {
      node.params.push_back(fromType(param, false));
    }
  } else {
    node = make(Kind::kOpaque, canonical.getUnqualifiedType().getAsString());
  }
  if (!quals_on_element) {
    node.is_const |= canonical.isConstQualified();
    node.is_volatile |= canonical.isVolatileQualified();
  }
  node.type = canonical;
  return node;
}

Node IrSrcBuilder::fromRecord(const clang::RecordDecl *decl) {
  if (stand_in) {
    if (auto param = stand_in(decl)) {
      Node node = make(Kind::kParam);
      node.param = *param;
      return node;
    }
  }
  if (const auto *cxx = llvm::dyn_cast<clang::CXXRecordDecl>(decl);
      cxx && cxx->isLambda()) {
    return make(Kind::kOpaque, "lambda");
  }
  Node node = make(Kind::kRecord, tagName(decl));
  if (const auto *spec =
          llvm::dyn_cast<clang::ClassTemplateSpecializationDecl>(decl)) {
    for (const auto &arg : spec->getTemplateArgs().asArray()) {
      node.args.push_back(fromTemplateArg(arg));
    }
  }
  return node;
}

Node IrSrcBuilder::fromTemplateArg(const clang::TemplateArgument &arg) {
  switch (arg.getKind()) {
  case clang::TemplateArgument::Type:
    return fromType(arg.getAsType(), false);
  case clang::TemplateArgument::Integral: {
    Node node = make(Kind::kValue, llvm::toString(arg.getAsIntegral(), 10));
    node.type = arg.getIntegralType();
    return node;
  }
  default: {
    std::string spelling;
    llvm::raw_string_ostream os(spelling);
    arg.print(clang::PrintingPolicy(ctx_.getLangOpts()), os,
              /*IncludeType=*/true);
    return make(Kind::kOpaque, spelling);
  }
  }
}

std::shared_ptr<Node> IrSrcBuilder::classOf(const clang::Decl *decl) {
  if (const auto *record =
          llvm::dyn_cast<clang::RecordDecl>(decl->getDeclContext())) {
    Node node = fromRecord(record);
    node.type = ctx_.getCanonicalTagType(record);
    return share(std::move(node));
  }
  return nullptr;
}

Node IrSrcBuilder::FromDecl(const clang::NamedDecl *decl) {
  if (const auto *tmpl = llvm::dyn_cast<clang::FunctionTemplateDecl>(decl)) {
    decl = tmpl->getTemplatedDecl();
  }
  const auto *func = llvm::dyn_cast<clang::FunctionDecl>(decl);
  if (!func) {
    Node node = make(Kind::kDecl, QualifiedName(decl));
    node.class_ = classOf(decl);
    return node;
  }

  Node node = make(Kind::kFunction, QualifiedName(func));
  node.variadic = func->isVariadic();
  node.class_ = classOf(func);
  node.return_type = share(fromType(func->getReturnType(), false));
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
    node.params.push_back(make(Kind::kPackParams));
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

std::optional<Node> IrSrcBuilder::FromExpr(const clang::Expr *expr) {
  expr = expr->IgnoreParenImpCasts();

  if (llvm::isa<clang::IntegerLiteral>(expr) &&
      expr->getBeginLoc().isMacroID()) {
    auto name = clang::Lexer::getImmediateMacroName(
        expr->getBeginLoc(), ctx_.getSourceManager(), ctx_.getLangOpts());
    if (!name.empty()) {
      return make(Kind::kMacro, name.str());
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
      Node node = make(Kind::kArrow);
      node.object = share(FromType(object));
      node.member = share(FromDecl(decl));
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
        make(Kind::kUnary,
             (uop->isPostfix() ? "post" : "") +
                 clang::UnaryOperator::getOpcodeStr(uop->getOpcode()).str());
    node.operand = share(std::move(*sub));
    return node;
  }

  return std::nullopt;
}

PushASTContext::PushASTContext(clang::ASTContext &ctx) : prev_(ctx_) {
  ctx_ = &ctx;
}
PushASTContext::~PushASTContext() { ctx_ = prev_; }

bool Contains(clang::QualType qual_type) {
  return search(qual_type).first != nullptr;
}

bool Contains(const clang::Expr *expr) { return search(expr).first != nullptr; }

const IrTgt::ExprRule *GetExprRule(const clang::Expr *expr) {
  auto rule = search(expr).first;
  return rule ? &rule->tgt : nullptr;
}

const IrSrc::InitTypeLocation &GetInitType(const clang::Expr *expr) {
  auto rule = search(expr).first;
  assert(rule && "expression must have a translation rule");
  return rule->src.init_type;
}

bool IsLibcPassthrough(const clang::Expr *expr) {
  const auto *tgt_ir = GetExprRule(expr);
  if (tgt_ir == nullptr || !tgt_ir->body.empty() || !tgt_ir->is_extern) {
    return false;
  }
  const auto *ref =
      clang::dyn_cast<clang::DeclRefExpr>(expr->IgnoreParenImpCasts());
  const auto *decl = ref != nullptr ? ref->getDecl() : nullptr;
  return decl != nullptr &&
         decl->getASTContext().getSourceManager().isInSystemHeader(
             decl->getLocation());
}

std::string MapFunctionName(const clang::FunctionDecl *decl) {
  assert(decl);
  if (!IsUserDefinedDecl(decl) &&
      exprs_.contains(IndexKey(IrSrcBuilder(*ctx_).FromDecl(decl)))) {
    return std::format("libcc2rs::{}_{}", decl->getNameAsString(),
                       model_ == Model::kRefCount ? "refcount" : "unsafe");
  }
  return GetNamedDeclAsString(decl->getCanonicalDecl());
}

std::string InstantiateTemplate(const clang::Expr *expr, unsigned n) {
  auto [rule, bindings] = search(expr);
  auto text = std::format("T{}", n);
  if (!rule) {
    return text;
  }
  return instantiateTgt(bindings, text);
}

std::string Map(clang::QualType qual_type) {
  auto [rule, bindings] = search(qual_type);
  if (rule) {
    return instantiateTgt(bindings, rule->type_info.type);
  }
  return {};
}

std::string MapInitializer(clang::QualType qual_type) {
  auto [rule, bindings] = search(qual_type);
  if (rule && !rule->initializer.empty()) {
    return instantiateTgt(bindings, rule->initializer);
  }
  return {};
}

bool MapsToPointer(clang::QualType qual_type) {
  auto rule = search(qual_type).first;
  return rule && rule->type_info.is_pointer();
}

bool MapsToRefcountPointer(clang::QualType qual_type) {
  auto rule = search(qual_type).first;
  return rule && rule->type_info.is_refcount_pointer;
}

const std::vector<std::string> *MappedDerives(clang::QualType qual_type) {
  auto rule = search(qual_type).first;
  return rule ? &rule->type_info.derives : nullptr;
}

void SetDerives(clang::QualType qual_type, std::vector<std::string> derives) {
  if (auto *rule = search(qual_type).first) {
    rule->type_info.derives = std::move(derives);
  }
}
bool ReturnsPointer(const clang::Expr *expr) {
  auto rule = search(expr).first;
  return rule && rule->tgt.return_type.is_pointer();
}

const IrTgt::TypeInfo &GetParamInfo(const clang::Expr *expr, unsigned index) {
  auto rule = search(expr).first;
  assert(rule && "expression must have a translation rule");
  return rule->tgt.params.at(index);
}

std::string GetParamType(const clang::Expr *expr, unsigned index) {
  auto [rule, bindings] = search(expr);
  assert(rule && "expression must have a translation rule");
  return instantiateTgt(bindings, rule->tgt.params.at(index).type);
}

bool ParamIsPointer(const clang::Expr *expr, unsigned index) {
  return GetParamInfo(expr, index).is_pointer();
}

clang::QualType GetTypeForDecl(const clang::NamedDecl *decl) {
  if (const auto *spec =
          llvm::dyn_cast<clang::ClassTemplateSpecializationDecl>(decl)) {
    llvm::ArrayRef<clang::TemplateArgument> args =
        spec->getTemplateArgs().asArray();
    llvm::SmallVector<clang::TemplateArgument, 4> canon(args.begin(),
                                                        args.end());
    ctx_->canonicalizeTemplateArguments(canon);

    return ctx_->getTemplateSpecializationType(
        clang::ElaboratedTypeKeyword::None,
        clang::TemplateName(spec->getSpecializedTemplate()), args, canon);
  }

  const auto *rdecl = llvm::dyn_cast<clang::TagDecl>(decl);
  assert(rdecl && "Unsupported decl type");

  return ctx_->getTagType(clang::ElaboratedTypeKeyword::None,
                          rdecl->getQualifier(), rdecl, /*OwnsTag*/ false);
}

void AddRuleForUserDefinedType(clang::NamedDecl *decl) {
  auto type = GetTypeForDecl(decl);
  auto ptr = ctx_->getPointerType(type);
  auto rs_name = ToRustName(ToString(type));

  AddTypeRule(type, IrTgt::TypeRule::Plain(rs_name));

  if (auto record_decl = llvm::dyn_cast<clang::RecordDecl>(decl)) {
    // Forward declaration
    if (!record_decl->isThisDeclarationADefinition()) {
      return;
    }

    if (auto cxx_decl = llvm::dyn_cast<clang::CXXRecordDecl>(record_decl)) {
      if (cxx_decl->isAbstract()) {
        switch (model_) {
        case Model::kUnsafe:
          AddTypeRule(ptr, IrTgt::TypeRule::UnsafePtr("*mut dyn " + rs_name));
          break;
        case Model::kRefCount:
          AddTypeRule(
              ptr, IrTgt::TypeRule::RefcountPtr("PtrDyn<dyn " + rs_name + '>'));
          break;
        }
      } else {
        switch (model_) {
        case Model::kUnsafe:
          AddTypeRule(ptr, IrTgt::TypeRule::UnsafePtr("*mut " + rs_name));
          break;
        case Model::kRefCount:
          AddTypeRule(ptr,
                      IrTgt::TypeRule::RefcountPtr("Ptr<" + rs_name + '>'));
          break;
        }
      }

      for (auto *nested : GetNestedStructs(cxx_decl)) {
        AddRuleForUserDefinedType(nested);
      }
    }
  }
}

std::string ToRustName(std::string name) {
  ReplaceAll(name, "::", "_");
  ReplaceAll(name, "*", "ptr");
  ReplaceAll(name, "&", "ref");
  ReplaceAll(name, "[", "arr");
  ReplaceAll(name, "]", "arr");
  ReplaceAll(name, "-", "neg");
  for (auto &c : name) {
    if (!std::isalnum(c) && c != '_') {
      c = '_';
    }
  }
  return name;
}

std::string ToString(clang::QualType qual_type, ScalarSugar sugar) {
  assert(ctx_);

  if (sugar == ScalarSugar::kPreserve) {
    clang::QualType t = qual_type;
    if (const auto *decltype_type =
            clang::dyn_cast<clang::DecltypeType>(t.getTypePtr())) {
      t = decltype_type->getUnderlyingType();
    }
    if (const auto *typedef_type = t->getAs<clang::TypedefType>()) {
      if (t.getCanonicalType()->isBuiltinType()) {
        return typedef_type->getDecl()->getNameAsString();
      }
    } else if (const auto *predef = t->getAs<clang::PredefinedSugarType>()) {
      return predef->getIdentifier()->getName().str();
    } else if (const auto *ptr = t->getAs<clang::PointerType>()) {
      auto pointee = ptr->getPointeeType();
      auto canonical = pointee.getCanonicalType().getDesugaredType(*ctx_);
      bool builtin_alias = canonical->isBuiltinType() &&
                           (pointee->getAs<clang::TypedefType>() ||
                            pointee->getAs<clang::PredefinedSugarType>());
      if (!builtin_alias && Map(pointee) == Map(canonical)) {
        pointee = canonical;
      }
      std::string out;
      llvm::raw_string_ostream os(out);
      ctx_->getPointerType(pointee).print(os, getPrintPolicy());
      return normalizeTranslationRule(std::move(out));
    }
  }

  if (auto cxx_record_decl = qual_type->getAsCXXRecordDecl()) {
    if (cxx_record_decl->isLambda()) {
      return ToString(cxx_record_decl->getLambdaCallOperator());
    }
  }

  if (auto *tag = qual_type->getAsTagDecl();
      tag && !tag->getIdentifier() && !tag->getTypedefNameForAnonDecl()) {
    return ToString(clang::cast<clang::NamedDecl>(tag));
  }

  if (auto *tag = qual_type->getAsTagDecl();
      tag && tag->getIdentifier() &&
      tag->getDeclContext()->isFunctionOrMethod()) {
    return GetNamedDeclAsString(tag);
  }

  if (auto renamed = DisambiguateAnonymousTag(qual_type->getAsTagDecl());
      !renamed.empty()) {
    return renamed;
  }

  std::string type;
  llvm::raw_string_ostream os(type);
  normalizeQualType(qual_type).print(os, getPrintPolicy());
  return normalizeTranslationRule(std::move(type));
}

bool HasFunctionParameterPack(const clang::FunctionDecl *decl) {
  if (auto *primary = decl->getPrimaryTemplate()) {
    decl = primary->getTemplatedDecl();
  }
  return decl->getNumParams() && decl->parameters().back()->isParameterPack();
}

std::string ToString(const clang::NamedDecl *decl) {
  if (auto *record = clang::dyn_cast<clang::RecordDecl>(decl);
      record && !record->getIdentifier()) {
    if (auto renamed = DisambiguateAnonymousTag(record); !renamed.empty()) {
      return renamed;
    }
    if (auto *typedef_decl = record->getTypedefNameForAnonDecl()) {
      return ToString(clang::cast<clang::NamedDecl>(typedef_decl));
    }
    return GetNamedDeclAsString(record);
  }

  if (auto *enum_decl = clang::dyn_cast<clang::EnumDecl>(decl)) {
    if (auto renamed = DisambiguateAnonymousTag(enum_decl); !renamed.empty()) {
      return renamed;
    }
    if (!enum_decl->getIdentifier() &&
        !enum_decl->getTypedefNameForAnonDecl()) {
      return GetNamedDeclAsString(enum_decl);
    }
  }

  std::string out;
  llvm::raw_string_ostream os(out);

  const clang::FunctionDecl *func_decl = nullptr;
  if (auto *template_decl = llvm::dyn_cast<clang::FunctionTemplateDecl>(decl)) {
    func_decl = template_decl->getTemplatedDecl();
  } else {
    func_decl = llvm::dyn_cast_or_null<clang::FunctionDecl>(decl);
  }

  if (!func_decl) {
    decl->printQualifiedName(os, getPrintPolicy());
    return normalizeTranslationRule(std::move(out));
  }

  os << ToString(func_decl->getReturnType()) << ' ';
  if (const auto op = func_decl->getOverloadedOperator();
      op >= clang::OverloadedOperatorKind::OO_LessLess &&
      op <= clang::OverloadedOperatorKind::OO_GreaterGreaterEqual) {
    // ensure matchTemplate does not consider these operator names when matching
    func_decl->getQualifier().print(os, getPrintPolicy());
    os << "operator ";
    switch (op) {
    case clang::OverloadedOperatorKind::OO_LessLess:
      os << "shl";
      break;
    case clang::OverloadedOperatorKind::OO_GreaterGreater:
      os << "shr";
      break;
    case clang::OverloadedOperatorKind::OO_LessLessEqual:
      os << "shleq";
      break;
    case clang::OverloadedOperatorKind::OO_GreaterGreaterEqual:
      os << "shreq";
      break;
    default:
      assert(0 && "Unexpected overloaded operator kind");
    }
  } else if (const auto *method_decl =
                 llvm::dyn_cast<clang::CXXMethodDecl>(func_decl)) {
    if (method_decl->getParent()->isLambda() &&
        method_decl->getOverloadedOperator() == clang::OO_Call) {
      func_decl->printName(os, getPrintPolicy());
    } else {
      func_decl->printQualifiedName(os, getPrintPolicy());
    }
  } else {
    func_decl->printQualifiedName(os, getPrintPolicy());
  }

  bool has_pack = HasFunctionParameterPack(func_decl);
  unsigned num_params = func_decl->getNumParams();
  if (has_pack) {
    const auto *primary = func_decl->getPrimaryTemplate();
    num_params =
        (primary ? primary->getTemplatedDecl() : func_decl)->getNumParams() - 1;
  }

  os << '(';
  for (unsigned i = 0; i < num_params; ++i) {
    if (i) {
      os << ", ";
    }
    os << ToString(func_decl->getParamDecl(i)->getType());
  }
  if (has_pack) {
    if (num_params) {
      os << ", ";
    }
    os << kPackMarker;
  }
  if (func_decl->isVariadic()) {
    if (func_decl->getNumParams()) {
      os << ", ";
    }
    os << "...";
  }
  os << ')';

  if (const auto *method_decl =
          llvm::dyn_cast<clang::CXXMethodDecl>(func_decl)) {
    if (method_decl->isConst()) {
      os << " const";
    }
    if (method_decl->isVolatile()) {
      os << " volatile";
    }
    switch (method_decl->getRefQualifier()) {
    case clang::RQ_LValue:
      os << " &";
      break;
    case clang::RQ_RValue:
      os << " &&";
      break;
    default:
      break;
    }
  }

  return normalizeTranslationRule(std::move(out));
}

std::string ToString(const clang::Expr *expr) {
  if (!expr) {
    assert(0 && "!expr");
  }

  expr = expr->IgnoreParenImpCasts();

  if (llvm::isa<clang::IntegerLiteral>(expr) &&
      expr->getBeginLoc().isMacroID()) {
    auto &sm = ctx_->getSourceManager();
    auto name = clang::Lexer::getImmediateMacroName(expr->getBeginLoc(), sm,
                                                    ctx_->getLangOpts());
    if (!name.empty()) {
      return name.str();
    }
  }

  if (const auto *CE = llvm::dyn_cast<clang::CallExpr>(expr)) {
    if (const auto *decl = CE->getDirectCallee()) {
      return ToString(decl);
    }
  }

  if (const auto *ctor = llvm::dyn_cast<clang::CXXConstructExpr>(expr)) {
    if (const auto *ctor_decl = ctor->getConstructor()) {
      return ToString(ctor_decl);
    }
    assert(0 && "expr is a CXXConstructExpr but could not get constructor");
  }

  if (const auto *ME = llvm::dyn_cast<clang::MemberExpr>(expr)) {
    if (const auto *member_decl =
            llvm::dyn_cast<clang::NamedDecl>(ME->getMemberDecl())) {
      if (const auto *method_decl =
              llvm::dyn_cast<clang::CXXMethodDecl>(member_decl)) {
        return ToString(method_decl);
      }
      if (ME->isArrow()) {
        auto *base = ME->getBase()->IgnoreParenImpCasts();
        if (auto *op = llvm::dyn_cast<clang::CXXOperatorCallExpr>(base)) {
          if (op->getOperator() == clang::OO_Arrow) {
            return ToString(op->getArg(0)->getType()) + "->" +
                   ToString(member_decl);
          }
        }
      } else if (auto for_range = GetParentForRange(*ctx_, ME)) {
        if (ToString(for_range->getRangeInit()->getType())
                .starts_with("std::map<")) {
          auto iter_type = GetForRangeIteratorType(for_range);
          if (!iter_type.isNull()) {
            return ToString(iter_type) + "->" + ToString(member_decl);
          }
        }
      }
      return ToString(member_decl);
    }
    assert(0 && "expr is a MemberExpr but could not get named decl");
  }

  if (const auto *decl_ref = llvm::dyn_cast<clang::DeclRefExpr>(expr)) {
    if (const auto *named_decl =
            llvm::dyn_cast<clang::NamedDecl>(decl_ref->getDecl())) {
      if (const auto *tmpl_decl =
              llvm::dyn_cast<clang::FunctionTemplateDecl>(named_decl)) {
        return ToString(tmpl_decl->getTemplatedDecl());
      }
      return ToString(named_decl);
    }
    return "";
  }

  if (const auto *uop = llvm::dyn_cast<clang::UnaryOperator>(expr)) {
    auto sub = ToString(uop->getSubExpr());
    std::string_view opcode =
        clang::UnaryOperator::getOpcodeStr(uop->getOpcode());
    return uop->isPostfix() ? std::format("{}{}", sub, opcode)
                            : std::format("{}{}", opcode, sub);
  }

  return "Unhandled case in ToString";
}

void LoadTranslationRules(Model model, clang::ASTContext &ctx,
                          const std::string &rules_dir) {
  ctx_ = &ctx;
  model_ = model;

  if (translation_rules_loaded_) {
    return;
  }
  translation_rules_loaded_ = true;

  addRulesFromDirectory(rules_dir, model);

#if 0
  for (auto &[src, rule] : exprs_) {
    log() << "Expr key: " << src << '\n';
    rule.dump();
  }
  for (auto &[src, rule] : types_) {
    log() << "Type key: " << src << '\n';
    rule.dump();
  }
#endif
}

} // namespace cpp2rust::Mapper
