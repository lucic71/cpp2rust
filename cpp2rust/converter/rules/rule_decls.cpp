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

#include "converter/printer.h"
#include "converter/rules/registry.h"
#include "logging.h"
#include "rules_prelude.h"

namespace cpp2rust::RuleDecls {

namespace {

constexpr std::string_view kPrefix = "cpp2rust_rules_";

ExprRuleDeclMap exprs_;
TypeRuleDeclMap types_;
TypeRuleDeclMap member_types_;
std::unordered_map<const void *, TranslationRule::TypeRule *> plain_types_;
std::unordered_map<std::string, TranslationRule::TypeRule *> alias_types_;
clang::Sema *sema_ = nullptr;

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

} // namespace

const clang::Expr *GetReturned(const clang::FunctionDecl *decl) {
  const auto *body =
      llvm::dyn_cast_or_null<clang::CompoundStmt>(decl->getBody());
  if (!body || body->size() != 1) {
    return nullptr;
  }
  const auto *ret = llvm::dyn_cast<clang::ReturnStmt>(*body->body_begin());
  const clang::Expr *returned = ret ? ret->getRetValue() : nullptr;
  while (returned) {
    const auto *construct =
        llvm::dyn_cast<clang::CXXConstructExpr>(SkipImplicit(returned));
    if (!construct || !construct->isElidable()) {
      break;
    }
    returned = construct->getArg(0);
  }
  return returned;
}

namespace {

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

} // namespace

std::string GetTypeKey(clang::QualType type) {
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

std::string GetAliasKey(clang::QualType type) {
  type = type.getUnqualifiedType();
  if (const auto *decltype_type =
          llvm::dyn_cast<clang::DecltypeType>(type.getTypePtr())) {
    type = decltype_type->getUnderlyingType().getUnqualifiedType();
  }
  if (const auto *typeof_type =
          llvm::dyn_cast<clang::TypeOfExprType>(type.getTypePtr())) {
    type = typeof_type->desugar().getUnqualifiedType();
  }
  if (const auto *typedef_type = type->getAs<clang::TypedefType>();
      typedef_type && type.getCanonicalType()->isBuiltinType()) {
    return typedef_type->getDecl()->getName().str();
  }
  if (const auto *predefined = type->getAs<clang::PredefinedSugarType>()) {
    return predefined->getIdentifier()->getName().str();
  }
  if (const auto *pointer = type->getAs<clang::PointerType>()) {
    auto pointee = pointer->getPointeeType();
    auto key = GetAliasKey(pointee);
    if (key.empty()) {
      return {};
    }
    return std::string(pointee.isConstQualified() ? "const " : "") +
           (pointee.isVolatileQualified() ? "volatile " : "") + key + " *";
  }
  return {};
}

const clang::Expr *SkipImplicit(const clang::Expr *expr) {
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
  }
}

namespace {

bool isDeclaredByRules(clang::ASTContext &ctx, const clang::Decl *decl) {
  auto &src_mgr = ctx.getSourceManager();
  return IsRulesPrelude(src_mgr.getFilename(
      src_mgr.getSpellingLoc(decl->getCanonicalDecl()->getLocation())));
}

bool refersToUndeclared(const clang::Expr *expr) {
  expr = SkipImplicit(expr);
  if (const auto *call = llvm::dyn_cast<clang::CallExpr>(expr)) {
    if (const auto *lookup = llvm::dyn_cast<clang::UnresolvedLookupExpr>(
            call->getCallee()->IgnoreParenImpCasts())) {
      return lookup->getName().isIdentifier() && lookup->getNumDecls() == 0;
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

} // namespace

std::string GetExprKey(clang::ASTContext &ctx, const clang::Expr *expr) {
  expr = SkipImplicit(expr);

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
    return GetExprKey(ctx, callee);
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
    return GetExprKey(ctx, unary->getSubExpr());
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
    return GetTypeKey(construct->getTypeAsWritten());
  }
  if (const auto *construct = llvm::dyn_cast<clang::CXXConstructExpr>(expr)) {
    return getNameKey(construct->getConstructor()->getParent()->getDeclName());
  }
  if (const auto *cast = llvm::dyn_cast<clang::ExplicitCastExpr>(expr)) {
    return GetTypeKey(cast->getTypeAsWritten());
  }
  return {};
}

namespace {

unsigned getGenericNumber(const clang::NamedDecl *param) {
  auto name = param->getName();
  unsigned n = 0;
  if (!name.consume_front("T") || name.getAsInteger(10, n)) {
    return 0;
  }
  return n;
}

clang::QualType getInitType(const clang::FunctionDecl *function) {
  if (function->param_empty()) {
    return {};
  }
  const auto *expansion = function->parameters()
                              .back()
                              ->getType()
                              ->getAs<clang::PackExpansionType>();
  if (!expansion) {
    return {};
  }
  const auto *alias = expansion->getPattern()
                          .getNonReferenceType()
                          ->getAs<clang::TemplateSpecializationType>();
  if (!alias || !alias->isTypeAlias() ||
      alias->getTemplateName().getAsTemplateDecl()->getName() != "Init") {
    return {};
  }
  return alias->template_arguments()[0].getAsType();
}

void checkGenerics(const RuleId &id, const clang::FunctionDecl *function,
                   const TranslationRule::ExprRule &rule) {
  bool declared[TranslationRule::kMaxGenerics + 1] = {false};
  if (const auto *tmpl = function->getDescribedFunctionTemplate()) {
    for (const auto *param : *tmpl->getTemplateParameters()) {
      auto n = getGenericNumber(param);
      assert(n <= TranslationRule::kMaxGenerics &&
             "template placeholder exceeds kMaxGenerics");
      declared[n] = true;
    }
  }
  for (size_t i = 0, e = rule.generics.size(); i < e; ++i) {
    if (!declared[i + 1]) {
      llvm::errs() << id.module << "::" << id.name << ": generic T" << (i + 1)
                   << " declared but missing from src\n";
      llvm::report_fatal_error("Absent generic from src");
    }
  }
  if (rule.usesInit() && getInitType(function).isNull()) {
    llvm::errs() << id.module << "::" << id.name << '\n';
    llvm::report_fatal_error(
        "Expr rule uses init but its src pack is not declared as Init<T, "
        "Args>");
  }
}

void addExprRule(clang::ASTContext &ctx, const clang::NamedDecl *decl) {
  auto id = getRuleId(decl);
  if (!id || !isRuleName(id->name, 'f')) {
    return;
  }
  if (const auto *var = llvm::dyn_cast<clang::VarDecl>(decl)) {
    const auto *init = var->getInit();
    auto *rule = RuleRegistry::FindExprRule(id->module, id->name);
    if (var->isInvalidDecl() || !init || init->containsErrors() || !rule) {
      log() << "rule " << id->module << "::" << id->name << " does not apply\n";
      return;
    }
    auto key = GetExprKey(ctx, init);
    log() << "rule " << id->module << "::" << id->name << " -> '" << key
          << "'\n";
    exprs_.emplace(std::move(key), ExprRuleDecl{nullptr, init, rule, {}});
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
  const auto *returned = GetReturned(function);
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
  checkGenerics(*id, function, *rule);
  auto key = GetExprKey(ctx, returned);
  log() << "rule " << id->module << "::" << id->name << " -> '" << key << "'\n";
  exprs_.emplace(std::move(key),
                 ExprRuleDecl{function, returned, rule, getInitType(function)});
}

void addTypeRule(clang::ASTContext &ctx, const clang::NamedDecl *decl) {
  auto id = getRuleId(decl);
  if (!id || !isRuleName(id->name, 't')) {
    return;
  }
  if (llvm::isa<clang::FunctionTemplateDecl>(decl)) {
    if (auto *rule = RuleRegistry::FindTypeRule(id->module, id->name)) {
      log() << "rule " << id->module << "::" << id->name << " -> ''\n";
      types_.emplace("", TypeRuleDecl{decl, rule});
    }
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
  if (!llvm::isa<clang::TypeAliasTemplateDecl>(decl)) {
    log() << "rule " << id->module << "::" << id->name << " is a plain type\n";
    auto add = [&](auto &types, auto key) {
      auto [it, inserted] = types.try_emplace(std::move(key), rule);
      if (!inserted && it->second != rule) {
        llvm::errs() << "ERROR: duplicate type rule for C++ type '"
                     << Printer::ToString(ctx, alias->getUnderlyingType())
                     << "': maps to both '" << it->second->type_info.type
                     << "' and '" << rule->type_info.type << "'\n";
        std::exit(EXIT_FAILURE);
      }
    };
    if (auto key = GetAliasKey(alias->getUnderlyingType()); !key.empty()) {
      add(alias_types_, std::move(key));
      return;
    }
    add(plain_types_, alias->getUnderlyingType()
                          .getCanonicalType()
                          .getUnqualifiedType()
                          .getAsOpaquePtr());
    return;
  }
  if (const auto *member =
          alias->getUnderlyingType()->getAs<clang::DependentNameType>()) {
    auto key = member->getIdentifier()->getName().str();
    log() << "rule " << id->module << "::" << id->name << " -> member '" << key
          << "'\n";
    member_types_.emplace(std::move(key), TypeRuleDecl{decl, rule});
    return;
  }
  auto key = GetTypeKey(alias->getUnderlyingType());
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

clang::Sema &GetSema() {
  assert(sema_);
  return *sema_;
}

void Collect(clang::Sema &sema) {
  auto &ctx = sema.Context;
  sema_ = &sema;
  exprs_.clear();
  types_.clear();
  member_types_.clear();
  plain_types_.clear();
  alias_types_.clear();
  addRules(ctx, ctx.getTranslationUnitDecl());
  log() << "collected " << exprs_.size() << " expression rules and "
        << types_.size() + plain_types_.size() + alias_types_.size()
        << " type rules\n";
}

TranslationRule::TypeRule *FindPlainType(clang::QualType type) {
  for (auto alias = type;;) {
    auto key = GetAliasKey(alias);
    if (key.empty()) {
      break;
    }
    if (auto it = alias_types_.find(key); it != alias_types_.end()) {
      return it->second;
    }
    const auto *typedef_type = alias->getAs<clang::TypedefType>();
    if (!typedef_type) {
      break;
    }
    alias = typedef_type->desugar();
  }
  auto it = plain_types_.find(
      type.getCanonicalType().getUnqualifiedType().getAsOpaquePtr());
  return it == plain_types_.end() ? nullptr : it->second;
}

std::ranges::subrange<ExprRuleDeclMap::iterator>
ExprCandidates(const std::string &key) {
  auto [begin, end] = exprs_.equal_range(key);
  return {begin, end};
}

std::ranges::subrange<TypeRuleDeclMap::iterator>
MemberTypeCandidates(const std::string &key) {
  auto [begin, end] = member_types_.equal_range(key);
  return {begin, end};
}

std::ranges::subrange<TypeRuleDeclMap::iterator>
TypeCandidates(const std::string &key) {
  auto [begin, end] = types_.equal_range(key);
  return {begin, end};
}

} // namespace cpp2rust::RuleDecls
