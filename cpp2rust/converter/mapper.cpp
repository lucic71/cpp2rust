// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/mapper.h"

#include <clang/AST/ExprCXX.h>
#include <clang/Basic/OperatorKinds.h>
#include <clang/Basic/SourceManager.h>
#include <clang/Lex/Lexer.h>
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
#include "converter/rules/match.h"

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

void AddTypeRule(clang::QualType type, IrTgt::TypeRule &&rule) {
  auto src = IrSrc::Builder(*ctx_).FromType(type);
  auto key = IrSrc::IndexKey(src);
  auto [begin, end] = types_.equal_range(key);
  for (auto it = begin; it != end; ++it) {
    if (it->second.src.ir == src) {
      return;
    }
  }
  types_.emplace(std::move(key), TypeRule{{std::move(src)}, std::move(rule)});
}

std::string instantiateTgt(const IrSrc::Bindings &bindings,
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
    if (bound.kind == IrSrc::Node::Kind::kValue) {
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
std::pair<T *, IrSrc::Bindings>
search(std::unordered_multimap<std::string, T> &map, const IrSrc::Node &use,
       const std::string &key) {
  auto [it, end] = map.equal_range(key);
  T *rule = nullptr;
  IrSrc::Bindings bindings;
  unsigned specificity = 0;
  for (; it != end; ++it) {
    IrSrc::Bindings these;
    const auto &ir = it->second.src.ir;
    if (!IrSrc::Match(ir, use, these)) {
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

std::pair<ExprRule *, IrSrc::Bindings> search(const clang::Expr *expr) {
  if (RefersToUserDefinedDecl(expr)) {
    return {};
  }
  auto use = IrSrc::Builder(*ctx_).FromExpr(expr);
  if (!use) {
    return {};
  }
  auto res = search(exprs_, *use, IrSrc::IndexKey(*use));
  log() << "search expr " << use->str() << ", result:\n";
  if (res.first) {
    res.first->src.dump();
    res.first->tgt.dump();
  } else {
    log() << "None\n";
  }
  return res;
}

IrSrc::Node typeIR(clang::QualType qual_type, bool sugared) {
  IrSrc::Builder builder(*ctx_);
  if (sugared) {
    builder.keep_builtin_typedef = true;
    builder.keep_pointee_sugar = [](clang::QualType pointee) {
      auto canonical = pointee.getCanonicalType().getDesugaredType(*ctx_);
      return Map(pointee) != Map(canonical);
    };
  }
  auto node = builder.FromType(qual_type);
  if (node.kind != IrSrc::Node::Kind::kArray &&
      node.kind != IrSrc::Node::Kind::kIncompleteArray) {
    node.is_const = false;
    node.is_volatile = false;
  }
  return node;
}

std::pair<IrTgt::TypeRule *, IrSrc::Bindings>
search(clang::QualType qual_type) {
  for (bool sugared : {true, false}) {
    auto use = typeIR(qual_type, sugared);
    auto key = IrSrc::IndexKey(use);
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
      auto key = IrSrc::IndexKey(paired.src.ir);
      exprs_.emplace(std::move(key), std::move(paired));
    }
    for (auto &[name, rule] : tgt.types) {
      TypeRule paired{takeSrc(src.types, name, path), std::move(rule)};
      auto key = IrSrc::IndexKey(paired.src.ir);
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

void addBuiltinTypes(Model model) {
  assert(ctx_);

  auto add_builtin_rule = [&](clang::QualType qt, const std::string &rust) {
    auto plain = IrTgt::TypeRule::Plain(rust);
    std::vector<std::string> derives = {"Copy",  "Clone",     "Default",
                                        "Debug", "PartialEq", "PartialOrd"};
    if (!(rust == "f32" || rust == "f64")) {
      derives.insert(derives.end(), {"Eq", "Ord", "Hash"});
    }
    plain.type_info.derives = std::move(derives);
    AddTypeRule(qt, std::move(plain));

    auto ptr = ctx_->getPointerType(qt);
    auto const_ptr = ctx_->getPointerType(qt.withConst());
    switch (model) {
    case Model::kUnsafe:
      AddTypeRule(ptr, IrTgt::TypeRule::UnsafePtr("*mut " + rust));
      AddTypeRule(const_ptr, IrTgt::TypeRule::UnsafePtr("*const " + rust));
      break;
    case Model::kRefCount:
      AddTypeRule(ptr, IrTgt::TypeRule::RefcountPtr("Ptr::<" + rust + ">"));
      AddTypeRule(const_ptr,
                  IrTgt::TypeRule::RefcountPtr("Ptr::<" + rust + ">"));
      break;
    }
  };

  auto build_rust_type = [&](clang::QualType qt) {
    unsigned bits = ctx_->getTypeSize(qt);
    char sign = qt->isSignedIntegerType() ? 'i' : 'u';
    return std::format("{}{}", sign, bits);
  };

  // Misc
  add_builtin_rule(ctx_->BoolTy, "bool");
  add_builtin_rule(ctx_->FloatTy, "f32");
  add_builtin_rule(ctx_->DoubleTy, "f64");

  auto void_ptr = ctx_->getPointerType(ctx_->VoidTy);
  auto const_void_ptr = ctx_->getPointerType(ctx_->VoidTy.withConst());
  switch (model) {
  case Model::kUnsafe:
    AddTypeRule(void_ptr, IrTgt::TypeRule::UnsafePtr("*mut ::libc::c_void"));
    AddTypeRule(const_void_ptr,
                IrTgt::TypeRule::UnsafePtr("*const ::libc::c_void"));
    break;
  case Model::kRefCount:
    AddTypeRule(void_ptr, IrTgt::TypeRule::RefcountPtr("AnyPtr"));
    AddTypeRule(const_void_ptr, IrTgt::TypeRule::RefcountPtr("AnyPtr"));
    break;
  }

  // Char
  switch (model) {
  case Model::kUnsafe:
    add_builtin_rule(ctx_->CharTy, "libc::c_char");
    break;
  case Model::kRefCount:
    add_builtin_rule(ctx_->CharTy, "u8");
    break;
  }
  add_builtin_rule(ctx_->SignedCharTy, "i8");
  add_builtin_rule(ctx_->UnsignedCharTy, "u8");

  // Integers
  add_builtin_rule(ctx_->ShortTy, build_rust_type(ctx_->ShortTy));
  add_builtin_rule(ctx_->UnsignedShortTy,
                   build_rust_type(ctx_->UnsignedShortTy));
  add_builtin_rule(ctx_->IntTy, build_rust_type(ctx_->IntTy));
  add_builtin_rule(ctx_->UnsignedIntTy, build_rust_type(ctx_->UnsignedIntTy));
  add_builtin_rule(ctx_->LongTy, build_rust_type(ctx_->LongTy));
  add_builtin_rule(ctx_->UnsignedLongTy, build_rust_type(ctx_->UnsignedLongTy));
  add_builtin_rule(ctx_->LongLongTy, build_rust_type(ctx_->LongLongTy));
  add_builtin_rule(ctx_->UnsignedLongLongTy,
                   build_rust_type(ctx_->UnsignedLongLongTy));
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
  return rule;
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
      exprs_.contains(IrSrc::IndexKey(IrSrc::Builder(*ctx_).FromDecl(decl)))) {
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
      if (Map(pointee) == Map(canonical)) {
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

  addBuiltinTypes(model);
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
