// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/mapper.h"

#include <clang/AST/ExprCXX.h>
#include <clang/Basic/OperatorKinds.h>
#include <clang/Basic/SourceManager.h>
#include <llvm/Support/ThreadPool.h>

#include <cctype>
#include <cstdlib>
#include <format>
#include <optional>
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

using Node = IrSrc::Node;
using Kind = Node::Kind;
using Bindings = std::vector<std::optional<Node>>;

bool Match(const Node &rule, const Node &use, Bindings &bindings) {
  if (rule.kind == Kind::kParam) {
    assert(rule.param < bindings.size());
    auto &slot = bindings[rule.param];
    // First time we see the binding, always succeed.
    if (!slot) {
      slot = use;
      return true;
    }
    // Second time we see the binding, check that it equals the first usage.
    return *slot == use;
  }
  if (!rule.shallowEquals(use)) {
    return false;
  }
  return Node::zipChildren(rule, use, [&](const Node &r, const Node &u) {
    return Match(r, u, bindings);
  });
}

void AddTypeRule(clang::QualType type, IrTgt::TypeRule &&rule) {
  auto src = IrSrc::Builder(*ctx_).FromType(type);
  auto key = src.indexKey();
  auto [begin, end] = types_.equal_range(key);
  for (auto it = begin; it != end; ++it) {
    if (it->second.src.ir == src) {
      // Skip if the rule already exists
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
    const auto &src = it->second.src;
    const auto &ir = src.ir;
    Bindings these(src.num_params + 1);
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
  auto use = IrSrc::Builder(*ctx_).FromExpr(expr);
  if (!use) {
    return {};
  }
  auto res = search(exprs_, *use, use->indexKey());
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
  IrSrc::Builder builder(*ctx_);
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
  while (node.kind == Kind::kConst || node.kind == Kind::kVolatile) {
    node = Node(*node.operand);
  }
  return node;
}

std::pair<IrTgt::TypeRule *, Bindings> search(clang::QualType qual_type) {
  for (bool sugared : {true, false}) {
    auto use = typeIR(qual_type, sugared);
    auto key = use.indexKey();
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
      auto key = paired.src.ir.indexKey();
      exprs_.emplace(std::move(key), std::move(paired));
    }
    for (auto &[name, rule] : tgt.types) {
      TypeRule paired{takeSrc(src.types, name, path), std::move(rule)};
      auto key = paired.src.ir.indexKey();
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

} // namespace

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
      exprs_.contains(IrSrc::Builder(*ctx_).FromDecl(decl).indexKey())) {
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
  auto type = ctx_->getCanonicalTagType(clang::cast<clang::TagDecl>(decl));
  auto ptr = ctx_->getPointerType(type);
  auto rs_name = ToRustName(ToString(GetTypeForDecl(decl)));

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

std::string ToString(clang::QualType qual_type) {
  assert(ctx_);

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
  qual_type.getCanonicalType().getUnqualifiedType().print(os, getPrintPolicy());
  return type;
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
    return out;
  }

  os << ToString(func_decl->getReturnType()) << ' ';
  if (const auto *method_decl =
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

  os << '(';
  for (unsigned i = 0, e = func_decl->getNumParams(); i < e; ++i) {
    if (i) {
      os << ", ";
    }
    os << ToString(func_decl->getParamDecl(i)->getType());
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

  return out;
}

std::string ToString(const clang::Expr *expr) {
  if (!expr) {
    assert(0 && "!expr");
  }

  expr = expr->IgnoreParenImpCasts();

  if (const auto *CE = llvm::dyn_cast<clang::CallExpr>(expr)) {
    if (const auto *decl = CE->getDirectCallee()) {
      return ToString(decl);
    }
  }

  if (const auto *ME = llvm::dyn_cast<clang::MemberExpr>(expr)) {
    if (const auto *member_decl =
            llvm::dyn_cast<clang::NamedDecl>(ME->getMemberDecl())) {
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
