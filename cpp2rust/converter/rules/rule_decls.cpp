// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/rule_decls.h"

#include <clang/AST/DeclCXX.h>
#include <clang/AST/DeclTemplate.h>
#include <clang/AST/ExprCXX.h>
#include <clang/Basic/OperatorKinds.h>
#include <clang/Basic/SourceManager.h>
#include <clang/Lex/Lexer.h>

#include <algorithm>
#include <cctype>
#include <optional>
#include <string_view>

#include "converter/rules/registry.h"
#include "logging.h"
#include "rules_prelude.h"

namespace cpp2rust::RuleDecls {

namespace {

constexpr std::string_view kPrefix = "cpp2rust_rules_";

ExprRuleDeclMap exprs_;
TypeRuleDeclMap types_;

bool isRuleName(std::string_view name, char kind) {
  return name.size() > 1 && name[0] == kind &&
         std::all_of(name.begin() + 1, name.end(),
                     [](char c) { return std::isdigit((unsigned char)c); });
}

struct RuleId {
  std::string module;
  std::string name;
};

std::optional<RuleId> getRuleId(const clang::NamedDecl *decl) {
  if (!decl->getIdentifier()) {
    return std::nullopt;
  }
  std::string_view name = decl->getName();
  if (const auto *ns =
          llvm::dyn_cast<clang::NamespaceDecl>(decl->getDeclContext())) {
    std::string_view module = ns->getIdentifier() ? ns->getName() : "";
    if (!module.starts_with(kPrefix)) {
      return std::nullopt;
    }
    return RuleId{std::string(module.substr(kPrefix.size())),
                  std::string(name)};
  }
  if (!name.starts_with(kPrefix)) {
    return std::nullopt;
  }
  name.remove_prefix(kPrefix.size());
  auto split = name.rfind('_');
  if (split == std::string_view::npos) {
    return std::nullopt;
  }
  return RuleId{std::string(name.substr(0, split)),
                std::string(name.substr(split + 1))};
}

const clang::Expr *getReturned(const clang::FunctionDecl *decl) {
  const auto *body =
      llvm::dyn_cast_or_null<clang::CompoundStmt>(decl->getBody());
  if (!body || body->size() != 1) {
    return nullptr;
  }
  const auto *ret = llvm::dyn_cast<clang::ReturnStmt>(*body->body_begin());
  return ret ? ret->getRetValue() : nullptr;
}

std::string getOperatorKey(clang::OverloadedOperatorKind op) {
  if (op == clang::OO_None) {
    return {};
  }
  return std::string("operator") + clang::getOperatorSpelling(op);
}

std::string getNameKey(clang::DeclarationName name) {
  switch (name.getNameKind()) {
  case clang::DeclarationName::Identifier:
    if (const auto *identifier = name.getAsIdentifierInfo()) {
      return identifier->getName().str();
    }
    return {};
  case clang::DeclarationName::CXXOperatorName:
    return getOperatorKey(name.getCXXOverloadedOperator());
  default:
    return {};
  }
}

std::string getTypeKey(clang::QualType type) {
  type = type.getNonReferenceType();
  if (const auto *tst = type->getAs<clang::TemplateSpecializationType>()) {
    if (const auto *tmpl = tst->getTemplateName().getAsTemplateDecl()) {
      return getNameKey(tmpl->getDeclName());
    }
  }
  if (const auto *tag = type->getAsTagDecl()) {
    return getNameKey(tag->getDeclName());
  }
  return {};
}

const clang::Expr *skipImplicit(const clang::Expr *expr) {
  while (true) {
    expr = expr->IgnoreParens();
    if (const auto *full = llvm::dyn_cast<clang::FullExpr>(expr)) {
      expr = full->getSubExpr();
      continue;
    }
    if (const auto *temporary =
            llvm::dyn_cast<clang::MaterializeTemporaryExpr>(expr)) {
      expr = temporary->getSubExpr();
      continue;
    }
    if (const auto *bind = llvm::dyn_cast<clang::CXXBindTemporaryExpr>(expr)) {
      expr = bind->getSubExpr();
      continue;
    }
    const auto *cast = llvm::dyn_cast<clang::ImplicitCastExpr>(expr);
    if (!cast) {
      return expr;
    }
    expr = cast->getSubExpr();
    if (cast->getCastKind() != clang::CK_UserDefinedConversion) {
      continue;
    }
    if (const auto *conversion =
            llvm::dyn_cast<clang::CXXMemberCallExpr>(expr->IgnoreImplicit())) {
      expr = conversion->getImplicitObjectArgument();
    }
  }
}

bool isDeclaredByRules(clang::ASTContext &ctx, const clang::Decl *decl) {
  auto &src_mgr = ctx.getSourceManager();
  return IsRulesPrelude(src_mgr.getFilename(
      src_mgr.getSpellingLoc(decl->getCanonicalDecl()->getLocation())));
}

bool refersToUndeclared(const clang::Expr *expr) {
  expr = skipImplicit(expr);
  if (const auto *call = llvm::dyn_cast<clang::CallExpr>(expr)) {
    if (const auto *lookup = llvm::dyn_cast<clang::UnresolvedLookupExpr>(
            call->getCallee()->IgnoreParenImpCasts())) {
      return lookup->getNumDecls() == 0;
    }
    if (const auto *callee = call->getDirectCallee()) {
      return callee->isImplicit() && !callee->getBuiltinID();
    }
  }
  return false;
}

bool refersToUndeclared(clang::ASTContext &ctx, clang::QualType type) {
  while (type->isPointerType() || type->isReferenceType()) {
    type = type->getPointeeType();
  }
  const auto *tag = type->getAsTagDecl();
  return tag && isDeclaredByRules(ctx, tag);
}

std::string getExprKey(clang::ASTContext &ctx, const clang::Expr *expr) {
  expr = skipImplicit(expr);

  if (llvm::isa<clang::IntegerLiteral>(expr) &&
      expr->getBeginLoc().isMacroID()) {
    return clang::Lexer::getImmediateMacroName(
               expr->getBeginLoc(), ctx.getSourceManager(), ctx.getLangOpts())
        .str();
  }
  if (const auto *op = llvm::dyn_cast<clang::CXXOperatorCallExpr>(expr)) {
    return getOperatorKey(op->getOperator());
  }
  if (const auto *rewritten =
          llvm::dyn_cast<clang::CXXRewrittenBinaryOperator>(expr)) {
    return getOperatorKey(
        clang::BinaryOperator::getOverloadedOperator(rewritten->getOperator()));
  }
  if (const auto *call = llvm::dyn_cast<clang::CallExpr>(expr)) {
    const auto *callee = call->getCallee()->IgnoreParenImpCasts();
    if (const auto *ref = llvm::dyn_cast<clang::DeclRefExpr>(callee);
        ref && llvm::isa<clang::ParmVarDecl>(ref->getDecl())) {
      return getOperatorKey(clang::OO_Call);
    }
    return getExprKey(ctx, callee);
  }
  if (const auto *overload = llvm::dyn_cast<clang::OverloadExpr>(expr)) {
    return getNameKey(overload->getName());
  }
  if (const auto *member =
          llvm::dyn_cast<clang::CXXDependentScopeMemberExpr>(expr)) {
    return getNameKey(member->getMember());
  }
  if (const auto *ref =
          llvm::dyn_cast<clang::DependentScopeDeclRefExpr>(expr)) {
    return getNameKey(ref->getDeclName());
  }
  if (const auto *member = llvm::dyn_cast<clang::MemberExpr>(expr)) {
    return getNameKey(member->getMemberDecl()->getDeclName());
  }
  if (const auto *ref = llvm::dyn_cast<clang::DeclRefExpr>(expr)) {
    return getNameKey(ref->getDecl()->getDeclName());
  }
  if (const auto *unary = llvm::dyn_cast<clang::UnaryOperator>(expr)) {
    if (unary->getSubExpr()->isTypeDependent()) {
      return getOperatorKey(
          clang::UnaryOperator::getOverloadedOperator(unary->getOpcode()));
    }
    return getExprKey(ctx, unary->getSubExpr());
  }
  if (const auto *binary = llvm::dyn_cast<clang::BinaryOperator>(expr)) {
    return getOperatorKey(
        clang::BinaryOperator::getOverloadedOperator(binary->getOpcode()));
  }
  if (llvm::isa<clang::ArraySubscriptExpr>(expr)) {
    return getOperatorKey(clang::OO_Subscript);
  }
  if (const auto *construct =
          llvm::dyn_cast<clang::CXXUnresolvedConstructExpr>(expr)) {
    return getTypeKey(construct->getTypeAsWritten());
  }
  if (const auto *construct = llvm::dyn_cast<clang::CXXConstructExpr>(expr)) {
    return getNameKey(construct->getConstructor()->getParent()->getDeclName());
  }
  if (const auto *cast = llvm::dyn_cast<clang::ExplicitCastExpr>(expr)) {
    return getTypeKey(cast->getTypeAsWritten());
  }
  return {};
}

void addExprRule(clang::ASTContext &ctx, const clang::NamedDecl *decl) {
  auto id = getRuleId(decl);
  if (!id || !isRuleName(id->name, 'f')) {
    return;
  }
  const clang::FunctionDecl *function = nullptr;
  if (const auto *tmpl = llvm::dyn_cast<clang::FunctionTemplateDecl>(decl)) {
    function = tmpl->getTemplatedDecl();
  } else {
    function = llvm::dyn_cast<clang::FunctionDecl>(decl);
  }
  if (!function || !function->isThisDeclarationADefinition()) {
    return;
  }
  const auto *returned = getReturned(function);
  if (decl->isInvalidDecl() || function->isInvalidDecl() || !returned ||
      returned->containsErrors() || refersToUndeclared(returned)) {
    log() << "rule " << id->module << "::" << id->name << " does not apply\n";
    return;
  }
  auto *rule = RuleRegistry::FindExprRule(id->module, id->name);
  if (!rule) {
    log() << "rule " << id->module << "::" << id->name << " has no target\n";
    return;
  }
  auto key = getExprKey(ctx, returned);
  log() << "rule " << id->module << "::" << id->name << " -> '" << key << "'\n";
  exprs_.emplace(std::move(key), ExprRuleDecl{function, rule});
}

void addTypeRule(clang::ASTContext &ctx, const clang::NamedDecl *decl) {
  auto id = getRuleId(decl);
  if (!id || !isRuleName(id->name, 't')) {
    return;
  }
  const clang::TypedefNameDecl *alias = nullptr;
  if (const auto *tmpl = llvm::dyn_cast<clang::TypeAliasTemplateDecl>(decl)) {
    alias = tmpl->getTemplatedDecl();
  } else {
    alias = llvm::dyn_cast<clang::TypedefNameDecl>(decl);
  }
  if (!alias) {
    return;
  }
  if (decl->isInvalidDecl() || alias->isInvalidDecl() ||
      alias->getUnderlyingType()->containsErrors() ||
      refersToUndeclared(ctx, alias->getUnderlyingType())) {
    log() << "rule " << id->module << "::" << id->name << " does not apply\n";
    return;
  }
  auto *rule = RuleRegistry::FindTypeRule(id->module, id->name);
  if (!rule) {
    log() << "rule " << id->module << "::" << id->name << " has no target\n";
    return;
  }
  auto key = getTypeKey(alias->getUnderlyingType());
  log() << "rule " << id->module << "::" << id->name << " -> '" << key << "'\n";
  types_.emplace(std::move(key), TypeRuleDecl{decl, rule});
}

void addRules(clang::ASTContext &ctx, const clang::DeclContext *context) {
  for (const auto *child : context->decls()) {
    if (const auto *ns = llvm::dyn_cast<clang::NamespaceDecl>(child)) {
      if (ns->getIdentifier() && ns->getName().starts_with(kPrefix)) {
        addRules(ctx, ns);
      }
      continue;
    }
    if (const auto *named = llvm::dyn_cast<clang::NamedDecl>(child)) {
      addExprRule(ctx, named);
      addTypeRule(ctx, named);
    }
  }
}

} // namespace

void Collect(clang::ASTContext &ctx) {
  exprs_.clear();
  types_.clear();
  addRules(ctx, ctx.getTranslationUnitDecl());
  log() << "collected " << exprs_.size() << " expression rules and "
        << types_.size() << " type rules\n";
}

std::ranges::subrange<ExprRuleDeclMap::iterator>
ExprCandidates(const std::string &key) {
  auto [begin, end] = exprs_.equal_range(key);
  return {begin, end};
}

std::ranges::subrange<TypeRuleDeclMap::iterator>
TypeCandidates(const std::string &key) {
  auto [begin, end] = types_.equal_range(key);
  return {begin, end};
}

} // namespace cpp2rust::RuleDecls
