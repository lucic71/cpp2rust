#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/Expr.h>
#include <clang/AST/Type.h>

#include <string>
#include <utility>
#include <vector>

#include "converter/translation_rule.h"

namespace cpp2rust::Matcher {
using Bindings = std::vector<clang::QualType>;

template <typename Rule> using Match = std::pair<Rule *, Bindings>;

Match<TranslationRule::ExprRule> Find(clang::ASTContext &ctx,
                                      const clang::Expr *expr);
TranslationRule::TypeRule *FindLoaded(clang::ASTContext &ctx,
                                      clang::QualType type);

Match<TranslationRule::TypeRule> Find(clang::ASTContext &ctx,
                                      clang::QualType type);

clang::QualType GetInitType(clang::ASTContext &ctx, const clang::Expr *expr);

bool HasRuleNamed(const clang::FunctionDecl *decl);

std::string MapBinding(clang::ASTContext &ctx, const Bindings &bindings,
                       unsigned n);

std::string InstantiateTgt(clang::ASTContext &ctx, const Bindings &types,
                           const std::string &tgt_template);
} // namespace cpp2rust::Matcher
