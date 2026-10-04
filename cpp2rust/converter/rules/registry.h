#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/Expr.h>
#include <clang/AST/Type.h>

#include <ranges>
#include <string>
#include <unordered_map>
#include <vector>

#include "converter/factory.h"
#include "converter/rules/matcher.h"
#include "converter/translation_rule.h"

namespace cpp2rust::RuleRegistry {
TranslationRule::ExprRule *FindExprRule(const std::string &dir,
                                        const std::string &name);

TranslationRule::TypeRule *FindTypeRule(const std::string &dir,
                                        const std::string &name);

TranslationRule::TypeRule *FindUserType(clang::ASTContext &ctx,
                                        clang::QualType type);

void ResetUserTypes();

Matcher::Match<TranslationRule::ExprRule> Search(clang::ASTContext &ctx,
                                                 const clang::Expr *expr);
Matcher::Match<TranslationRule::TypeRule> Search(clang::ASTContext &ctx,
                                                 clang::QualType qual_type);

const TranslationRule::ExprRule *GetExprRule(clang::ASTContext &ctx,
                                             const clang::Expr *expr);
bool IsLibcPassthrough(clang::ASTContext &ctx, const clang::Expr *expr);
bool ReturnsPointer(clang::ASTContext &ctx, const clang::Expr *expr);
bool ParamIsPointer(clang::ASTContext &ctx, const clang::Expr *expr,
                    unsigned index);
bool MapsToPointer(clang::ASTContext &ctx, clang::QualType qual_type);
bool MapsToRefcountPointer(clang::ASTContext &ctx, clang::QualType qual_type);

Model CurrentModel();

void Load(Model model, const std::string &rules_dir);
} // namespace cpp2rust::RuleRegistry
