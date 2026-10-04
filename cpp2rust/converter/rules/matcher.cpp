// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/matcher.h"

#include <clang/AST/DeclCXX.h>
#include <clang/AST/DeclTemplate.h>
#include <clang/AST/ExprCXX.h>
#include <clang/AST/ParentMapContext.h>
#include <clang/Lex/Lexer.h>
#include <clang/Sema/EnterExpressionEvaluationContext.h>
#include <clang/Sema/Sema.h>
#include <clang/Sema/Template.h>
#include <clang/Sema/TemplateDeduction.h>

#include <cassert>
#include <cctype>
#include <vector>

#include "converter/converter.h"
#include "converter/converter_lib.h"
#include "converter/mapper.h"
#include "converter/printer.h"
#include "converter/rules/registry.h"
#include "converter/rules/rule_decls.h"
#include "logging.h"

namespace cpp2rust::Matcher {

namespace {

using ExprMatch = Match<TranslationRule::ExprRule>;
using TypeMatch = Match<TranslationRule::TypeRule>;

struct Target {
  const clang::Decl *decl = nullptr;
  std::string macro;
  int unary = -1;
  int spelled = -1;

  bool operator==(const Target &) const = default;
  explicit operator bool() const { return decl || !macro.empty(); }
};

Target getTarget(clang::ASTContext &ctx, const clang::Expr *expr) {
  expr = RuleDecls::SkipImplicit(expr);

  if (llvm::isa<clang::IntegerLiteral>(expr) &&
      expr->getBeginLoc().isMacroID()) {
    Target target;
    target.macro = clang::Lexer::getImmediateMacroName(expr->getBeginLoc(),
                                                       ctx.getSourceManager(),
                                                       ctx.getLangOpts())
                       .str();
    return target;
  }
  if (const auto *rewritten =
          llvm::dyn_cast<clang::CXXRewrittenBinaryOperator>(expr)) {
    auto target = getTarget(ctx, rewritten->getSemanticForm());
    target.spelled = rewritten->getOperator();
    return target;
  }
  if (const auto *unary = llvm::dyn_cast<clang::UnaryOperator>(expr)) {
    auto target = getTarget(ctx, unary->getSubExpr());
    target.unary = unary->getOpcode();
    return target;
  }
  Target target;
  if (const auto *call = llvm::dyn_cast<clang::CallExpr>(expr)) {
    if (const auto *callee = call->getDirectCallee()) {
      target.decl = callee->getCanonicalDecl();
    }
  } else if (const auto *construct =
                 llvm::dyn_cast<clang::CXXConstructExpr>(expr)) {
    target.decl = construct->getConstructor()->getCanonicalDecl();
  } else if (const auto *member = llvm::dyn_cast<clang::MemberExpr>(expr)) {
    target.decl = member->getMemberDecl()->getCanonicalDecl();
  } else if (const auto *ref = llvm::dyn_cast<clang::DeclRefExpr>(expr)) {
    target.decl = ref->getDecl()->getCanonicalDecl();
  } else if (const auto *cast = llvm::dyn_cast<clang::ExplicitCastExpr>(expr)) {
    return getTarget(ctx, cast->getSubExpr());
  }
  return target;
}

clang::Expr *getObject(clang::ASTContext &ctx, clang::Expr *object) {
  auto type = object->getType();
  if (type->isPointerType()) {
    type = type->getPointeeType();
  } else if (!type.isConstQualified()) {
    return object;
  }
  return new (ctx) clang::OpaqueValueExpr(
      object->getBeginLoc(), type.getUnqualifiedType(), clang::VK_LValue);
}

std::vector<clang::Expr *> getOperands(clang::ASTContext &ctx,
                                       const clang::Expr *use) {
  auto *expr = const_cast<clang::Expr *>(use);
  std::vector<clang::Expr *> operands;
  if (auto *rewritten =
          llvm::dyn_cast<clang::CXXRewrittenBinaryOperator>(expr)) {
    auto decomposed = rewritten->getDecomposedForm();
    operands = {const_cast<clang::Expr *>(decomposed.LHS),
                const_cast<clang::Expr *>(decomposed.RHS)};
  } else if (auto *op = llvm::dyn_cast<clang::CXXOperatorCallExpr>(expr)) {
    operands.assign(op->arg_begin(), op->arg_end());
    if (!operands.empty() &&
        llvm::isa_and_nonnull<clang::CXXMethodDecl>(op->getDirectCallee())) {
      operands[0] = operands[0]->IgnoreParenImpCasts();
    }
  } else if (auto *call = llvm::dyn_cast<clang::CXXMemberCallExpr>(expr)) {
    operands.push_back(getObject(
        ctx, call->getImplicitObjectArgument()->IgnoreParenImpCasts()));
    operands.insert(operands.end(), call->arg_begin(), call->arg_end());
  } else if (auto *call = llvm::dyn_cast<clang::CallExpr>(expr)) {
    if (const auto *method = llvm::dyn_cast_or_null<clang::CXXMethodDecl>(
            call->getDirectCallee());
        method && method->isStatic()) {
      operands.push_back(new (ctx) clang::OpaqueValueExpr(
          call->getBeginLoc(), ctx.getCanonicalTagType(method->getParent()),
          clang::VK_LValue));
    }
    operands.insert(operands.end(), call->arg_begin(), call->arg_end());
  } else if (auto *construct = llvm::dyn_cast<clang::CXXConstructExpr>(expr)) {
    operands.assign(construct->arg_begin(), construct->arg_end());
  } else if (auto *member = llvm::dyn_cast<clang::MemberExpr>(expr)) {
    auto *base = member->getBase()->IgnoreParenImpCasts();
    if (auto *arrow = llvm::dyn_cast<clang::CXXOperatorCallExpr>(base);
        arrow && arrow->getOperator() == clang::OO_Arrow) {
      base = arrow->getArg(0);
    } else if (auto for_range = GetParentForRange(ctx, member);
               for_range && !member->isArrow() &&
               GetClassName(for_range->getRangeInit()->getType()) ==
                   "std::map") {
      if (auto type = GetForRangeIteratorType(for_range); !type.isNull()) {
        base = new (ctx) clang::OpaqueValueExpr(member->getBeginLoc(), type,
                                                clang::VK_LValue);
      }
    }
    operands.push_back(getObject(ctx, base));
  }
  while (!operands.empty() &&
         llvm::isa<clang::CXXDefaultArgExpr>(operands.back())) {
    operands.pop_back();
  }
  return operands;
}

class Trial {
public:
  explicit Trial(clang::Sema &sema)
      : diagnostics_(sema.getDiagnostics()),
        suppressed_(diagnostics_.getSuppressAllDiagnostics()),
        trap_(sema, /*ForValidityCheck=*/true) {
    diagnostics_.setSuppressAllDiagnostics(true);
  }
  ~Trial() { diagnostics_.setSuppressAllDiagnostics(suppressed_); }

  bool failed() const { return trap_.hasErrorOccurred(); }

private:
  clang::DiagnosticsEngine &diagnostics_;
  bool suppressed_;
  clang::Sema::SFINAETrap trap_;
};

clang::FunctionDecl *deduce(clang::Sema &sema,
                            clang::FunctionTemplateDecl *rule,
                            clang::TemplateArgumentListInfo *explicit_args,
                            llvm::ArrayRef<clang::Expr *> operands,
                            clang::SourceLocation loc,
                            clang::TemplateDeductionResult &result) {
  clang::FunctionDecl *specialization = nullptr;
  clang::sema::TemplateDeductionInfo info(loc);
  auto check = [](llvm::ArrayRef<clang::QualType>, bool) { return false; };
  result = sema.DeduceTemplateArguments(
      rule, explicit_args, operands, specialization, info, false, false, false,
      clang::QualType(), clang::Expr::Classification(), false, check);
  return result == clang::TemplateDeductionResult::Success ? specialization
                                                           : nullptr;
}

clang::FunctionDecl *
deduceWithLeading(clang::Sema &sema, clang::FunctionTemplateDecl *rule,
                  llvm::ArrayRef<clang::TemplateArgument> leading,
                  llvm::ArrayRef<clang::Expr *> operands,
                  clang::SourceLocation loc) {
  clang::TemplateArgumentListInfo explicit_args(loc, loc);
  for (const auto &arg : leading) {
    if (arg.isNull()) {
      break;
    }
    auto type = arg.getKind() == clang::TemplateArgument::Integral
                    ? arg.getIntegralType()
                    : clang::QualType();
    explicit_args.addArgument(
        sema.getTrivialTemplateArgumentLoc(arg, type, loc));
  }
  if (explicit_args.size() == 0) {
    return nullptr;
  }
  clang::TemplateDeductionResult result;
  return deduce(sema, rule, &explicit_args, operands, loc, result);
}

std::vector<clang::TemplateArgument>
getArgsFromCallee(clang::FunctionTemplateDecl *rule, const Target &want) {
  std::vector<clang::TemplateArgument> args(
      rule->getTemplateParameters()->size());
  const auto *callee = llvm::dyn_cast_or_null<clang::FunctionDecl>(want.decl);
  const auto *callee_args =
      callee ? callee->getTemplateSpecializationArgs() : nullptr;
  const auto *returned = RuleDecls::GetReturned(rule->getTemplatedDecl());
  const auto *call =
      llvm::dyn_cast<clang::CallExpr>(RuleDecls::SkipImplicit(returned));
  const auto *written = call ? llvm::dyn_cast<clang::OverloadExpr>(
                                   call->getCallee()->IgnoreParenImpCasts())
                             : nullptr;
  if (!callee_args || !written || !written->hasExplicitTemplateArgs()) {
    return args;
  }
  unsigned position = 0;
  for (const auto &arg : written->template_arguments()) {
    if (position >= callee_args->size()) {
      break;
    }
    int index = -1;
    const auto &value = arg.getArgument();
    if (value.getKind() == clang::TemplateArgument::Type) {
      if (const auto *param =
              value.getAsType()->getAs<clang::TemplateTypeParmType>()) {
        index = param->getIndex();
      }
    } else if (value.getKind() == clang::TemplateArgument::Expression) {
      if (const auto *ref = llvm::dyn_cast<clang::DeclRefExpr>(
              value.getAsExpr()->IgnoreParenImpCasts())) {
        if (const auto *param = llvm::dyn_cast<clang::NonTypeTemplateParmDecl>(
                ref->getDecl())) {
          index = param->getIndex();
        }
      }
    }
    if (index >= 0 && (unsigned)index < args.size()) {
      args[index] = callee_args->get(position);
    }
    ++position;
  }
  return args;
}

std::vector<clang::TemplateArgument>
getArgsFromResultType(clang::Sema &sema, clang::FunctionTemplateDecl *rule,
                      const clang::Expr *use) {
  auto *params = rule->getTemplateParameters();
  llvm::SmallVector<clang::DeducedTemplateArgument, 4> deduced(params->size());
  clang::sema::TemplateDeductionInfo info(use->getBeginLoc());
  clang::TemplateArgument pattern(rule->getTemplatedDecl()->getReturnType());
  clang::TemplateArgument actual(use->getType().getCanonicalType());
  if (sema.DeduceTemplateArguments(params, pattern, actual, info, deduced,
                                   true) !=
      clang::TemplateDeductionResult::Success) {
    return {};
  }
  return {deduced.begin(), deduced.end()};
}

clang::Expr *substituteReturned(clang::Sema &sema,
                                clang::FunctionTemplateDecl *rule,
                                clang::FunctionDecl *specialization,
                                clang::SourceLocation loc) {
  auto *pattern = rule->getTemplatedDecl();
  const auto *returned = RuleDecls::GetReturned(pattern);
  clang::EnterExpressionEvaluationContext unevaluated(
      sema, clang::Sema::ExpressionEvaluationContext::Unevaluated);
  clang::Sema::InstantiatingTemplate instantiating(sema, loc, specialization);
  clang::Sema::ContextRAII context(sema, specialization);
  clang::LocalInstantiationScope scope(sema);
  for (unsigned i = 0, j = 0; i < pattern->getNumParams(); ++i) {
    auto *param = pattern->getParamDecl(i);
    if (!param->isParameterPack()) {
      scope.InstantiatedLocal(param, specialization->getParamDecl(j++));
      continue;
    }
    scope.MakeInstantiatedLocalArgPack(param);
    for (; j < specialization->getNumParams(); ++j) {
      scope.InstantiatedLocalPackArg(param, specialization->getParamDecl(j));
    }
  }
  auto result =
      sema.SubstExpr(const_cast<clang::Expr *>(returned),
                     sema.getTemplateInstantiationArgs(specialization));
  return result.isInvalid() ? nullptr : result.get();
}

bool resolvesTo(clang::Sema &sema, clang::FunctionTemplateDecl *rule,
                clang::FunctionDecl *specialization, const Target &want,
                clang::SourceLocation loc) {
  if (!specialization) {
    return false;
  }
  Trial trial(sema);
  auto *returned = substituteReturned(sema, rule, specialization, loc);
  return returned && !trial.failed() &&
         getTarget(sema.Context, returned) == want;
}

clang::FunctionDecl *matchTemplate(clang::Sema &sema,
                                   clang::FunctionTemplateDecl *rule,
                                   const clang::Expr *use, const Target &want,
                                   llvm::ArrayRef<clang::Expr *> operands) {
  auto loc = use->getBeginLoc();
  clang::FunctionDecl *specialization = nullptr;
  {
    Trial trial(sema);
    clang::TemplateDeductionResult result;
    specialization = deduce(sema, rule, nullptr, operands, loc, result);
  }
  if (resolvesTo(sema, rule, specialization, want, loc)) {
    return specialization;
  }
  {
    Trial trial(sema);
    specialization = deduceWithLeading(
        sema, rule, getArgsFromCallee(rule, want), operands, loc);
  }
  if (resolvesTo(sema, rule, specialization, want, loc)) {
    return specialization;
  }
  {
    Trial trial(sema);
    specialization = deduceWithLeading(
        sema, rule, getArgsFromResultType(sema, rule, use), operands, loc);
  }
  if (resolvesTo(sema, rule, specialization, want, loc)) {
    return specialization;
  }
  return nullptr;
}

unsigned getGenericNumber(const clang::NamedDecl *param) {
  auto name = param->getName();
  unsigned n = 0;
  if (!name.consume_front("T") || name.getAsInteger(10, n)) {
    return 0;
  }
  return n;
}

Bindings getBindings(const clang::TemplateParameterList *params,
                     llvm::ArrayRef<clang::TemplateArgument> args) {
  Bindings bindings;
  for (unsigned i = 0; i < params->size() && i < args.size(); ++i) {
    auto n = getGenericNumber(params->getParam(i));
    if (!n || args[i].getKind() != clang::TemplateArgument::Type) {
      continue;
    }
    if (bindings.size() < n) {
      bindings.resize(n);
    }
    bindings[n - 1] = args[i].getAsType();
  }
  return bindings;
}

struct Candidate {
  TranslationRule::ExprRule *rule = nullptr;
  clang::QualType init_type;
  clang::FunctionTemplateDecl *tmpl = nullptr;
  clang::FunctionDecl *specialization = nullptr;
};

bool isBetter(clang::Sema &sema, const Candidate &a, const Candidate &b,
              clang::SourceLocation loc, unsigned num_operands) {
  if (!a.tmpl || !b.tmpl) {
    return !a.tmpl && b.tmpl;
  }
  Trial trial(sema);
  if (auto *better = sema.getMoreSpecializedTemplate(
          a.tmpl, b.tmpl, loc, clang::TPOC_Call, num_operands)) {
    return better == a.tmpl;
  }
  return a.tmpl->getTemplateParameters()->size() <
         b.tmpl->getTemplateParameters()->size();
}

ExprMatch findExpr(clang::ASTContext &ctx, const clang::Expr *expr,
                   clang::QualType *init_type) {
  auto &sema = RuleDecls::GetSema();
  const auto *use = expr;
  auto want = getTarget(ctx, use);
  if (!want) {
    return {};
  }
  auto operands = getOperands(ctx, use);
  auto key = RuleDecls::GetExprKey(ctx, use);

  Candidate best;
  auto consider = [&](const std::string &bucket) {
    for (auto &[_, entry] : RuleDecls::ExprCandidates(bucket)) {
      Candidate candidate{entry.rule, entry.init_type};
      auto *function = const_cast<clang::FunctionDecl *>(entry.decl);
      if (auto *tmpl =
              function ? function->getDescribedFunctionTemplate() : nullptr) {
        candidate.tmpl = tmpl;
        candidate.specialization =
            matchTemplate(sema, tmpl, use, want, operands);
        if (!candidate.specialization) {
          continue;
        }
      } else if (getTarget(ctx, entry.returned) != want) {
        continue;
      }
      if (!best.rule || isBetter(sema, candidate, best, use->getBeginLoc(),
                                 operands.size())) {
        best = candidate;
      }
    }
  };
  consider(key);
  if (!key.empty()) {
    consider("");
  }

  log() << "search expr '" << key << "', result:\n";
  if (!best.rule) {
    log() << "None\n";
    return {};
  }
  best.rule->dump();
  if (!best.tmpl) {
    return {best.rule, {}};
  }
  if (init_type && !best.init_type.isNull()) {
    Trial trial(sema);
    clang::Sema::InstantiatingTemplate instantiating(sema, use->getBeginLoc(),
                                                     best.specialization);
    *init_type = sema.SubstType(
        best.init_type, sema.getTemplateInstantiationArgs(best.specialization),
        use->getBeginLoc(), clang::DeclarationName());
  }
  return {best.rule,
          getBindings(
              best.tmpl->getTemplateParameters(),
              best.specialization->getTemplateSpecializationArgs()->asArray())};
}

bool isInstanceOf(clang::Sema &sema, const clang::TypeAliasTemplateDecl *a,
                  const clang::TypeAliasTemplateDecl *b) {
  auto *params = b->getTemplateParameters();
  llvm::SmallVector<clang::DeducedTemplateArgument, 4> deduced(params->size());
  clang::sema::TemplateDeductionInfo info(a->getLocation());
  clang::TemplateArgument pattern(b->getTemplatedDecl()->getUnderlyingType());
  clang::TemplateArgument actual(
      a->getTemplatedDecl()->getUnderlyingType().getCanonicalType());
  return sema.DeduceTemplateArguments(params, pattern, actual, info, deduced,
                                      true) ==
         clang::TemplateDeductionResult::Success;
}

TypeMatch findMemberType(clang::ASTContext &ctx, clang::QualType type) {
  auto &sema = RuleDecls::GetSema();
  while (const auto *typedef_type = type->getAs<clang::TypedefType>()) {
    const auto *decl = typedef_type->getDecl();
    if (const auto *owner =
            llvm::dyn_cast<clang::ClassTemplateSpecializationDecl>(
                decl->getDeclContext())) {
      for (auto &[_, entry] :
           RuleDecls::MemberTypeCandidates(decl->getName().str())) {
        const auto *alias =
            llvm::cast<clang::TypeAliasTemplateDecl>(entry.decl);
        const auto *member = alias->getTemplatedDecl()
                                 ->getUnderlyingType()
                                 ->getAs<clang::DependentNameType>();
        const auto *pattern = member->getQualifier().getAsType();
        if (!pattern) {
          continue;
        }
        auto *params = alias->getTemplateParameters();
        llvm::SmallVector<clang::DeducedTemplateArgument, 4> deduced(
            params->size());
        clang::sema::TemplateDeductionInfo info(alias->getLocation());
        Trial trial(sema);
        if (sema.DeduceTemplateArguments(
                params, clang::TemplateArgument(clang::QualType(pattern, 0)),
                clang::TemplateArgument(ctx.getCanonicalTagType(owner)), info,
                deduced, true) == clang::TemplateDeductionResult::Success) {
          return {entry.rule,
                  getBindings(params, std::vector<clang::TemplateArgument>(
                                          deduced.begin(), deduced.end()))};
        }
      }
    }
    type = typedef_type->desugar();
  }
  return {};
}

TypeMatch findType(clang::ASTContext &ctx, clang::QualType type) {
  if (auto match = findMemberType(ctx, type); match.first) {
    return match;
  }
  if (auto *rule = RuleDecls::FindPlainType(type)) {
    return {rule, {}};
  }

  auto &sema = RuleDecls::GetSema();
  auto canonical = type.getCanonicalType().getUnqualifiedType();
  TranslationRule::TypeRule *best = nullptr;
  const clang::TypeAliasTemplateDecl *best_alias = nullptr;
  Bindings best_bindings;
  auto consider = [&](const std::string &bucket) {
    for (auto &[_, entry] : RuleDecls::TypeCandidates(bucket)) {
      if (const auto *function =
              llvm::dyn_cast<clang::FunctionTemplateDecl>(entry.decl)) {
        if (best) {
          continue;
        }
        auto *tmpl = const_cast<clang::FunctionTemplateDecl *>(function);
        Trial trial(sema);
        clang::OpaqueValueExpr operand(tmpl->getLocation(),
                                       ctx.getPointerType(canonical),
                                       clang::VK_PRValue);
        clang::TemplateDeductionResult result;
        if (auto *specialization = deduce(sema, tmpl, nullptr, {&operand},
                                          tmpl->getLocation(), result)) {
          best = entry.rule;
          best_bindings = getBindings(
              tmpl->getTemplateParameters(),
              specialization->getTemplateSpecializationArgs()->asArray());
        }
        continue;
      }
      auto *alias = const_cast<clang::TypeAliasTemplateDecl *>(
          llvm::cast<clang::TypeAliasTemplateDecl>(entry.decl));
      Trial trial(sema);
      clang::sema::TemplateDeductionInfo info(alias->getLocation());
      if (sema.DeduceTemplateArgumentsFromType(alias, canonical, info) !=
          clang::TemplateDeductionResult::Success) {
        continue;
      }
      auto *params = alias->getTemplateParameters();
      llvm::SmallVector<clang::DeducedTemplateArgument, 4> deduced(
          params->size());
      clang::TemplateArgument pattern(
          alias->getTemplatedDecl()->getUnderlyingType());
      clang::TemplateArgument actual(canonical);
      if (sema.DeduceTemplateArguments(params, pattern, actual, info, deduced,
                                       true) !=
          clang::TemplateDeductionResult::Success) {
        continue;
      }
      if (best_alias && !(isInstanceOf(sema, alias, best_alias) &&
                          !isInstanceOf(sema, best_alias, alias))) {
        continue;
      }
      best = entry.rule;
      best_alias = alias;
      best_bindings = getBindings(params, std::vector<clang::TemplateArgument>(
                                              deduced.begin(), deduced.end()));
    }
  };
  auto key = RuleDecls::GetTypeKey(canonical);
  consider(key);
  if (!key.empty()) {
    consider("");
  }
  return {best, std::move(best_bindings)};
}

} // namespace

clang::QualType GetInitType(clang::ASTContext &ctx, const clang::Expr *expr) {
  clang::QualType type;
  findExpr(ctx, expr, &type);
  assert(!type.isNull() && "expression must match a rule with an init value");
  return type;
}

ExprMatch Find(clang::ASTContext &ctx, const clang::Expr *expr) {
  return findExpr(ctx, expr, nullptr);
}

TypeMatch Find(clang::ASTContext &ctx, clang::QualType type) {
  auto match = findType(ctx, type);
  log() << "search type " << Printer::ToString(ctx, type)
        << ", result: " << (match.first ? match.first->type_info.type : "None")
        << '\n';
  return match;
}

bool HasRuleNamed(const clang::FunctionDecl *decl) {
  return decl->getIdentifier() &&
         !RuleDecls::ExprCandidates(decl->getName().str()).empty();
}

std::string MapBinding(Converter &converter, const Bindings &bindings,
                       unsigned n) {
  auto type = bindings.at(n);
  assert(!type.isNull() && "template parameter is not bound to a type");
  return converter.GetUnboxedTypeAsString(type);
}

//
// Example:
//   tgt_template = "Vec<T1>"
//   result       = "Vec<i32>"
std::string InstantiateTgt(Converter &converter, const Bindings &types,
                           const std::string &tgt_template) {
  assert(types.size() <= TranslationRule::kMaxGenerics &&
         "template placeholder exceeds kMaxGenerics");
  std::string instantiated_template = tgt_template;
  std::string::size_type pos = 0;
  while ((pos = instantiated_template.find('T', pos)) != std::string::npos) {
    if (pos + 1 >= instantiated_template.size()) {
      break;
    }
    if (!std::isdigit(instantiated_template[pos + 1])) {
      ++pos;
      continue;
    }
    auto repl =
        MapBinding(converter, types, instantiated_template[pos + 1] - '1');
    instantiated_template.replace(pos, 2, repl);
    pos += repl.length();
  }
  return instantiated_template;
}

} // namespace cpp2rust::Matcher
