#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/Expr.h>
#include <clang/AST/Type.h>

#include <functional>
#include <optional>

#include "converter/rules/ir_src.h"

namespace cpp2rust::IrSrc {

class Builder {
public:
  explicit Builder(clang::ASTContext &ctx) : ctx_(ctx) {}

  std::function<std::optional<unsigned>(const clang::Decl *)> stand_in;
  bool keep_builtin_typedef = false;
  std::function<bool(clang::QualType pointee)> keep_pointee_sugar;

  NodePtr FromType(clang::QualType type);
  NodePtr FromDecl(const clang::NamedDecl *decl);
  NodePtr FromExpr(const clang::Expr *expr);

private:
  clang::ASTContext &ctx_;

  NodePtr fromType(clang::QualType type, bool top);
  NodePtr fromCanonical(clang::QualType canonical);
  NodePtr fromTemplateArg(const clang::TemplateArgument &arg);
  NodePtr fromRecord(const clang::RecordDecl *decl);
  NodePtr classOf(const clang::Decl *decl);
};

} // namespace cpp2rust::IrSrc
