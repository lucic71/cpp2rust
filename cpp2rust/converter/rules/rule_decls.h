#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/Expr.h>
#include <clang/Sema/Sema.h>

#include <ranges>
#include <string>
#include <unordered_map>

#include "converter/translation_rule.h"

namespace cpp2rust::RuleDecls {
struct ExprRuleDecl {
  const clang::FunctionDecl *decl;
  const clang::Expr *returned;
  TranslationRule::ExprRule *rule;
  clang::QualType init_type;
};

struct TypeRuleDecl {
  const clang::NamedDecl *decl;
  TranslationRule::TypeRule *rule;
};

using ExprRuleDeclMap = std::unordered_multimap<std::string, ExprRuleDecl>;
using TypeRuleDeclMap = std::unordered_multimap<std::string, TypeRuleDecl>;

void Collect(clang::Sema &sema);

clang::Sema &GetSema();

const clang::Expr *GetReturned(const clang::FunctionDecl *decl);

const clang::Expr *SkipImplicit(const clang::Expr *expr);

std::string GetExprKey(clang::ASTContext &ctx, const clang::Expr *expr);

std::string GetTypeKey(clang::QualType type);

std::string GetAliasKey(clang::QualType type);

TranslationRule::TypeRule *FindPlainType(clang::QualType type);

std::ranges::subrange<ExprRuleDeclMap::iterator>
ExprCandidates(const std::string &key);

std::ranges::subrange<TypeRuleDeclMap::iterator>
TypeCandidates(const std::string &key);

std::ranges::subrange<TypeRuleDeclMap::iterator>
MemberTypeCandidates(const std::string &key);
} // namespace cpp2rust::RuleDecls
