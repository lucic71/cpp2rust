#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>

#include <ranges>
#include <string>
#include <unordered_map>

#include "converter/translation_rule.h"

namespace cpp2rust::RuleDecls {
struct ExprRuleDecl {
  const clang::FunctionDecl *decl;
  TranslationRule::ExprRule *rule;
};

struct TypeRuleDecl {
  const clang::NamedDecl *decl;
  TranslationRule::TypeRule *rule;
};

using ExprRuleDeclMap = std::unordered_multimap<std::string, ExprRuleDecl>;
using TypeRuleDeclMap = std::unordered_multimap<std::string, TypeRuleDecl>;

void Collect(clang::ASTContext &ctx);

std::ranges::subrange<ExprRuleDeclMap::iterator>
ExprCandidates(const std::string &key);

std::ranges::subrange<TypeRuleDeclMap::iterator>
TypeCandidates(const std::string &key);
} // namespace cpp2rust::RuleDecls
