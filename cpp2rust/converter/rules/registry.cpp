// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/registry.h"

#include <clang/Basic/SourceManager.h>

#include <algorithm>
#include <cstdlib>
#include <filesystem>
#include <utility>

#include "converter/converter_lib.h"
#include "converter/printer.h"

namespace cpp2rust::RuleRegistry {

namespace {

Model model_ = Model::kUnsafe;
bool translation_rules_loaded_ = false;

std::unordered_map<std::string, TranslationRule::ExprRule> exprs_;
std::unordered_map<std::string, TranslationRule::TypeRule> types_;

std::unordered_map<const void *, TranslationRule::TypeRule> user_types_;

void AddTypeRule(clang::QualType type, TranslationRule::TypeRule &&rule) {
  user_types_.try_emplace(type.getCanonicalType().getAsOpaquePtr(),
                          std::move(rule));
}

void addRulesFromDirectory(const std::filesystem::path &dir, Model model) {
  namespace fs = std::filesystem;
  for (const auto &entry : fs::directory_iterator(dir)) {
    const auto &path = entry.path();
    assert(fs::exists(path / "ir_unsafe.json") ||
           fs::exists(path / "ir_refcount.json"));
    auto [expr_rules, type_rules] = TranslationRule::Load(path, model);
    if (expr_rules.empty() && type_rules.empty()) {
      log() << "No rules found in " << path << '\n';
      continue;
    }
    auto module = path.filename().string();
    for (auto &[name, rule] : expr_rules) {
      exprs_.emplace(module + '/' + name, std::move(rule));
    }
    for (auto &[name, rule] : type_rules) {
      types_.emplace(module + '/' + name, std::move(rule));
    }
  }
}

const TranslationRule::TypeInfo &
GetParamInfo(clang::ASTContext &ctx, const clang::Expr *expr, unsigned index) {
  auto rule = Search(ctx, expr).first;
  assert(rule && "expression must have a translation rule");
  return rule->params.at(index);
}

} // namespace

TranslationRule::ExprRule *FindExprRule(const std::string &module,
                                        const std::string &name) {
  auto it = exprs_.find(module + '/' + name);
  return it == exprs_.end() ? nullptr : &it->second;
}

TranslationRule::TypeRule *FindTypeRule(const std::string &module,
                                        const std::string &name) {
  auto it = types_.find(module + '/' + name);
  return it == types_.end() ? nullptr : &it->second;
}

TranslationRule::TypeRule *FindUserType(clang::QualType type) {
  auto it = user_types_.find(
      type.getCanonicalType().getUnqualifiedType().getAsOpaquePtr());
  return it == user_types_.end() ? nullptr : &it->second;
}

void ResetUserTypes() { user_types_.clear(); }

Matcher::Match<TranslationRule::ExprRule> Search(clang::ASTContext &ctx,
                                                 const clang::Expr *expr) {
  if (RefersToUserDefinedDecl(expr)) {
    return {};
  }
  return Matcher::Find(ctx, expr);
}

Matcher::Match<TranslationRule::TypeRule> Search(clang::ASTContext &ctx,
                                                 clang::QualType qual_type) {
  return Matcher::Find(ctx, qual_type);
}

const TranslationRule::ExprRule *GetExprRule(clang::ASTContext &ctx,
                                             const clang::Expr *expr) {
  return Search(ctx, expr).first;
}

bool MapsToPointer(clang::ASTContext &ctx, clang::QualType qual_type) {
  auto rule = Search(ctx, qual_type).first;
  return rule && rule->type_info.is_pointer();
}

bool MapsToRefcountPointer(clang::ASTContext &ctx, clang::QualType qual_type) {
  auto rule = Search(ctx, qual_type).first;
  return rule && rule->type_info.is_refcount_pointer;
}

const std::vector<std::string> *MappedDerives(clang::ASTContext &ctx,
                                              clang::QualType qual_type) {
  auto rule = Search(ctx, qual_type).first;
  return rule ? &rule->type_info.derives : nullptr;
}

void SetDerives(clang::ASTContext &ctx, clang::QualType qual_type,
                std::vector<std::string> derives) {
  if (auto *rule = Search(ctx, qual_type).first) {
    rule->type_info.derives = std::move(derives);
  }
}

bool ReturnsPointer(clang::ASTContext &ctx, const clang::Expr *expr) {
  auto rule = Search(ctx, expr).first;
  return rule && rule->return_type.is_pointer();
}

bool ParamIsPointer(clang::ASTContext &ctx, const clang::Expr *expr,
                    unsigned index) {
  return GetParamInfo(ctx, expr, index).is_pointer();
}

bool IsLibcPassthrough(clang::ASTContext &ctx, const clang::Expr *expr) {
  const auto *tgt_ir = GetExprRule(ctx, expr);
  if (tgt_ir == nullptr || !tgt_ir->body.empty() || !tgt_ir->is_extern) {
    return false;
  }
  if (const auto *call = clang::dyn_cast<clang::CallExpr>(expr)) {
    expr = call->getCallee();
  }
  const auto *ref =
      clang::dyn_cast<clang::DeclRefExpr>(expr->IgnoreParenImpCasts());
  const auto *decl = ref != nullptr ? ref->getDecl() : nullptr;
  return decl != nullptr &&
         decl->getASTContext().getSourceManager().isInSystemHeader(
             decl->getLocation());
}

Model CurrentModel() { return model_; }

void AddRuleForUserDefinedType(clang::ASTContext &ctx, clang::NamedDecl *decl) {
  auto type = ctx.getCanonicalTagType(llvm::cast<clang::TagDecl>(decl));
  auto ptr = ctx.getPointerType(type);
  auto rs_name =
      Printer::ToRustName(Printer::ToString(ctx, GetTypeForDecl(ctx, decl)));

  AddTypeRule(type, TranslationRule::TypeRule::Plain(rs_name));

  if (auto record_decl = llvm::dyn_cast<clang::RecordDecl>(decl)) {
    // Forward declaration
    if (!record_decl->isThisDeclarationADefinition()) {
      return;
    }

    if (auto cxx_decl = llvm::dyn_cast<clang::CXXRecordDecl>(record_decl)) {
      if (cxx_decl->isAbstract()) {
        switch (model_) {
        case Model::kUnsafe:
          AddTypeRule(
              ptr, TranslationRule::TypeRule::UnsafePtr("*mut dyn " + rs_name));
          break;
        case Model::kRefCount:
          AddTypeRule(ptr, TranslationRule::TypeRule::RefcountPtr(
                               "PtrDyn<dyn " + rs_name + '>'));
          break;
        }
      } else {
        switch (model_) {
        case Model::kUnsafe:
          AddTypeRule(ptr,
                      TranslationRule::TypeRule::UnsafePtr("*mut " + rs_name));
          break;
        case Model::kRefCount:
          AddTypeRule(ptr, TranslationRule::TypeRule::RefcountPtr(
                               "Ptr<" + rs_name + '>'));
          break;
        }
      }

      for (auto *nested : GetNestedStructs(cxx_decl)) {
        AddRuleForUserDefinedType(ctx, nested);
      }
    }
  }
}

void Load(Model model, const std::string &rules_dir) {
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

} // namespace cpp2rust::RuleRegistry
