#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Expr.h>
#include <clang/AST/Type.h>

#include <functional>
#include <optional>
#include <string>

#include "converter/factory.h"
#include "converter/rules/ir_src.h"
#include "converter/rules/ir_tgt.h"

namespace cpp2rust::Mapper {
class PushASTContext {
public:
  explicit PushASTContext(clang::ASTContext &ctx);
  ~PushASTContext();
  PushASTContext(const PushASTContext &) = delete;
  PushASTContext &operator=(const PushASTContext &) = delete;

private:
  clang::ASTContext *prev_;
};

class IrSrcBuilder {
public:
  explicit IrSrcBuilder(clang::ASTContext &ctx) : ctx_(ctx) {}

  std::function<std::optional<unsigned>(const clang::Decl *)> stand_in;
  bool keep_builtin_typedef = false;
  std::function<bool(clang::QualType pointee)> keep_pointee_sugar;

  IrSrc::Node FromType(clang::QualType type);
  IrSrc::Node FromDecl(const clang::NamedDecl *decl);
  std::optional<IrSrc::Node> FromExpr(const clang::Expr *expr);

private:
  clang::ASTContext &ctx_;

  IrSrc::Node fromType(clang::QualType type, bool top);
  IrSrc::Node fromCanonical(clang::QualType canonical);
  IrSrc::Node fromTemplateArg(const clang::TemplateArgument &arg);
  IrSrc::Node fromRecord(const clang::RecordDecl *decl);
  std::shared_ptr<IrSrc::Node> classOf(const clang::Decl *decl);
};

bool Contains(clang::QualType qual_type);
bool Contains(const clang::Expr *expr);

std::string Map(clang::QualType qual_type);
std::string MapInitializer(clang::QualType qual_type);
const IrTgt::ExprRule *GetExprRule(const clang::Expr *expr);
const IrSrc::InitTypeLocation &GetInitType(const clang::Expr *expr);
bool IsLibcPassthrough(const clang::Expr *expr);
std::string MapFunctionName(const clang::FunctionDecl *decl);
std::string InstantiateTemplate(const clang::Expr *expr, unsigned n);
bool ReturnsPointer(const clang::Expr *expr);
std::string GetParamType(const clang::Expr *expr, unsigned index);
bool ParamIsPointer(const clang::Expr *expr, unsigned index);
bool MapsToPointer(clang::QualType qual_type);
bool MapsToRefcountPointer(clang::QualType qual_type);
const std::vector<std::string> *MappedDerives(clang::QualType qual_type);
void SetDerives(clang::QualType qual_type, std::vector<std::string> derives);

enum class ScalarSugar {
  kDesugar,
  kPreserve,
};

bool HasFunctionParameterPack(const clang::FunctionDecl *decl);

clang::QualType GetTypeForDecl(const clang::NamedDecl *decl);
std::string ToString(clang::QualType qual_type,
                     ScalarSugar sugar = ScalarSugar::kDesugar);
std::string ToString(const clang::Expr *expr);
std::string ToString(const clang::NamedDecl *decl);
std::string ToRustName(std::string name);

void LoadTranslationRules(Model model, clang::ASTContext &ctx,
                          const std::string &rules_dir);
void AddRuleForUserDefinedType(clang::NamedDecl *decl);
} // namespace cpp2rust::Mapper
