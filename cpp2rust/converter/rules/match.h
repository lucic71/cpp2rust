#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/Expr.h>
#include <clang/AST/Type.h>

#include <functional>
#include <optional>
#include <string>
#include <vector>

#include "converter/rules/ir_src.h"

namespace cpp2rust::IrSrc {

std::string IndexKey(const Node &node);

class Builder {
public:
  explicit Builder(clang::ASTContext &ctx) : ctx_(ctx) {}

  std::function<std::optional<unsigned>(const clang::Decl *)> stand_in;
  bool keep_builtin_typedef = false;
  std::function<bool(clang::QualType pointee)> keep_pointee_sugar;

  Node FromType(clang::QualType type);
  Node FromDecl(const clang::NamedDecl *decl);
  std::optional<Node> FromExpr(const clang::Expr *expr);

private:
  clang::ASTContext &ctx_;

  Node fromType(clang::QualType type, bool top);
  Node fromCanonical(clang::QualType canonical);
  Node fromTemplateArg(const clang::TemplateArgument &arg);
  Node fromRecord(const clang::RecordDecl *decl);
  Node classOf(const clang::Decl *decl);
};

std::string QualifiedName(const clang::NamedDecl *decl);

using Bindings = std::vector<std::optional<Node>>;

bool Match(const Node &rule, const Node &use, Bindings &bindings);

} // namespace cpp2rust::IrSrc
