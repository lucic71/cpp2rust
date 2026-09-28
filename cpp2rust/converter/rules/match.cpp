// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/match.h"

#include <clang/AST/DeclCXX.h>
#include <clang/AST/DeclTemplate.h>
#include <clang/AST/ExprCXX.h>
#include <clang/AST/PrettyPrinter.h>
#include <clang/Lex/Lexer.h>

#include <algorithm>
#include <cassert>

#include "converter/converter_lib.h"
#include "converter/mapper.h"

namespace cpp2rust::IrSrc {

namespace {

using Kind = Node::Kind;

std::shared_ptr<Node> share(Node node) {
  return std::make_shared<Node>(std::move(node));
}

Node make(Kind kind, std::string name = {}) {
  Node node;
  node.kind = kind;
  node.name = std::move(name);
  return node;
}

std::string nameOf(const clang::NamedDecl *decl) {
  if (const auto *ctor = llvm::dyn_cast<clang::CXXConstructorDecl>(decl)) {
    return ctor->getParent()->getName().str();
  }
  if (const auto *dtor = llvm::dyn_cast<clang::CXXDestructorDecl>(decl)) {
    return "~" + dtor->getParent()->getName().str();
  }
  auto name = decl->getDeclName();
  if (name.getNameKind() == clang::DeclarationName::CXXConversionFunctionName) {
    return "operator conversion";
  }
  if (const auto *tag = llvm::dyn_cast<clang::TagDecl>(decl);
      tag && !tag->getIdentifier()) {
    if (const auto *tdef = tag->getTypedefNameForAnonDecl()) {
      return tdef->getName().str();
    }
    return "(anonymous)";
  }
  return name.getAsString();
}

} // namespace

std::string QualifiedName(const clang::NamedDecl *decl) {
  std::vector<std::string> parts{nameOf(decl)};
  for (const auto *dc = decl->getDeclContext(); dc; dc = dc->getParent()) {
    if (const auto *ns = llvm::dyn_cast<clang::NamespaceDecl>(dc)) {
      if (!ns->isInline() && !ns->isAnonymousNamespace()) {
        parts.push_back(ns->getName().str());
      }
    } else if (const auto *named = llvm::dyn_cast<clang::NamedDecl>(dc);
               named && (llvm::isa<clang::RecordDecl>(named) ||
                         llvm::isa<clang::FunctionDecl>(named))) {
      parts.push_back(nameOf(named));
    }
  }
  std::string out;
  for (auto it = parts.rbegin(); it != parts.rend(); ++it) {
    if (!out.empty()) {
      out += "::";
    }
    out += *it;
  }
  return out;
}

std::string IndexKey(const Node &node) {
  switch (node.kind) {
  case Kind::kFunction:
  case Kind::kDecl:
  case Kind::kRecord:
  case Kind::kEnum:
  case Kind::kTypedef:
  case Kind::kBuiltin:
    return node.name;
  case Kind::kMacro:
    return "macro:" + node.name;
  case Kind::kUnary:
    return "unary" + node.name + ":" + IndexKey(*node.operand);
  case Kind::kArrow:
    return IndexKey(*node.member);
  case Kind::kPointer:
    return "*" + IndexKey(*node.pointee);
  case Kind::kLRef:
    return "&" + IndexKey(*node.pointee);
  case Kind::kRRef:
    return "&&" + IndexKey(*node.pointee);
  case Kind::kArray:
  case Kind::kIncompleteArray:
    return "[]" + IndexKey(*node.element);
  default:
    return "";
  }
}

Node Builder::FromType(clang::QualType type) { return fromType(type, true); }

Node Builder::fromType(clang::QualType type, bool top) {
  if (keep_builtin_typedef && top) {
    if (const auto *decltype_type =
            llvm::dyn_cast<clang::DecltypeType>(type.getTypePtr())) {
      type = decltype_type->getUnderlyingType();
    }
    auto canonical = type.getCanonicalType();
    const clang::NamedDecl *name = nullptr;
    if (const auto *typedef_type = type->getAs<clang::TypedefType>();
        typedef_type && canonical->isBuiltinType()) {
      name = typedef_type->getDecl();
    }
    if (name) {
      Node node = make(Kind::kTypedef, name->getName().str());
      node.is_const = canonical.isConstQualified();
      node.is_volatile = canonical.isVolatileQualified();
      node.type = canonical;
      return node;
    }
    if (const auto *predef = type->getAs<clang::PredefinedSugarType>()) {
      Node node =
          make(Kind::kTypedef, predef->getIdentifier()->getName().str());
      node.is_const = canonical.isConstQualified();
      node.is_volatile = canonical.isVolatileQualified();
      node.type = canonical;
      return node;
    }
    if (const auto *ptr = type->getAs<clang::PointerType>();
        ptr && keep_pointee_sugar &&
        keep_pointee_sugar(ptr->getPointeeType())) {
      Node node = make(Kind::kPointer);
      node.pointee = share(fromType(ptr->getPointeeType(), true));
      node.is_const = canonical.isConstQualified();
      node.is_volatile = canonical.isVolatileQualified();
      node.type = canonical;
      return node;
    }
  }
  return fromCanonical(type.getCanonicalType());
}

Node Builder::fromCanonical(clang::QualType canonical) {
  Node node;
  const auto *type = canonical.getTypePtr();
  bool quals_on_element = false;
  if (const auto *builtin = llvm::dyn_cast<clang::BuiltinType>(type)) {
    clang::PrintingPolicy policy(ctx_.getLangOpts());
    policy.Bool = true;
    node = make(Kind::kBuiltin, builtin->getName(policy).str());
  } else if (const auto *ptr = llvm::dyn_cast<clang::PointerType>(type)) {
    node = make(Kind::kPointer);
    node.pointee = share(fromType(ptr->getPointeeType(), false));
  } else if (const auto *ref =
                 llvm::dyn_cast<clang::LValueReferenceType>(type)) {
    node = make(Kind::kLRef);
    node.pointee = share(fromType(ref->getPointeeType(), false));
  } else if (const auto *ref =
                 llvm::dyn_cast<clang::RValueReferenceType>(type)) {
    node = make(Kind::kRRef);
    node.pointee = share(fromType(ref->getPointeeType(), false));
  } else if (const auto *array = ctx_.getAsConstantArrayType(canonical)) {
    node = make(Kind::kArray);
    node.element = share(fromType(array->getElementType(), false));
    node.size =
        share(make(Kind::kValue, llvm::toString(array->getSize(), 10, false)));
    quals_on_element = true;
  } else if (const auto *array = ctx_.getAsIncompleteArrayType(canonical)) {
    node = make(Kind::kIncompleteArray);
    node.element = share(fromType(array->getElementType(), false));
    quals_on_element = true;
  } else if (const auto *record = type->getAsRecordDecl()) {
    node = fromRecord(record);
  } else if (const auto *enum_type = llvm::dyn_cast<clang::EnumType>(type)) {
    node = make(Kind::kEnum, QualifiedName(enum_type->getDecl()));
  } else if (const auto *proto =
                 llvm::dyn_cast<clang::FunctionProtoType>(type)) {
    node = make(Kind::kFunctionType);
    node.variadic = proto->isVariadic();
    node.return_type = share(fromType(proto->getReturnType(), false));
    for (auto param : proto->getParamTypes()) {
      node.params.push_back(fromType(param, false));
    }
  } else {
    node = make(Kind::kOpaque, canonical.getUnqualifiedType().getAsString());
  }
  if (!quals_on_element) {
    node.is_const |= canonical.isConstQualified();
    node.is_volatile |= canonical.isVolatileQualified();
  }
  node.type = canonical;
  return node;
}

Node Builder::fromRecord(const clang::RecordDecl *decl) {
  if (stand_in) {
    if (auto param = stand_in(decl)) {
      Node node = make(Kind::kParam);
      node.param = *param;
      return node;
    }
  }
  if (const auto *cxx = llvm::dyn_cast<clang::CXXRecordDecl>(decl);
      cxx && cxx->isLambda()) {
    return make(Kind::kOpaque, "lambda");
  }
  Node node = make(Kind::kRecord, QualifiedName(decl));
  if (const auto *spec =
          llvm::dyn_cast<clang::ClassTemplateSpecializationDecl>(decl)) {
    for (const auto &arg : spec->getTemplateArgs().asArray()) {
      node.args.push_back(fromTemplateArg(arg));
    }
  }
  return node;
}

Node Builder::fromTemplateArg(const clang::TemplateArgument &arg) {
  switch (arg.getKind()) {
  case clang::TemplateArgument::Type:
    return fromType(arg.getAsType(), false);
  case clang::TemplateArgument::Integral: {
    Node node = make(Kind::kValue, llvm::toString(arg.getAsIntegral(), 10));
    node.type = arg.getIntegralType();
    return node;
  }
  case clang::TemplateArgument::Pack: {
    Node node = make(Kind::kPack);
    for (const auto &element : arg.pack_elements()) {
      node.args.push_back(fromTemplateArg(element));
    }
    return node;
  }
  default: {
    std::string spelling;
    llvm::raw_string_ostream os(spelling);
    arg.print(clang::PrintingPolicy(ctx_.getLangOpts()), os,
              /*IncludeType=*/true);
    return make(Kind::kOpaque, spelling);
  }
  }
}

std::shared_ptr<Node> Builder::classOf(const clang::Decl *decl) {
  if (const auto *record =
          llvm::dyn_cast<clang::RecordDecl>(decl->getDeclContext())) {
    Node node = fromRecord(record);
    node.type = ctx_.getCanonicalTagType(record);
    return share(std::move(node));
  }
  return nullptr;
}

Node Builder::FromDecl(const clang::NamedDecl *decl) {
  if (const auto *tmpl = llvm::dyn_cast<clang::FunctionTemplateDecl>(decl)) {
    decl = tmpl->getTemplatedDecl();
  }
  const auto *func = llvm::dyn_cast<clang::FunctionDecl>(decl);
  if (!func) {
    Node node = make(Kind::kDecl, QualifiedName(decl));
    node.class_ = classOf(decl);
    return node;
  }

  Node node = make(Kind::kFunction, QualifiedName(func));
  node.variadic = func->isVariadic();
  node.class_ = classOf(func);
  node.return_type = share(fromType(func->getReturnType(), false));
  bool has_pack = Mapper::HasFunctionParameterPack(func);
  unsigned num_params = func->getNumParams();
  if (has_pack) {
    const auto *primary = func->getPrimaryTemplate();
    num_params =
        (primary ? primary->getTemplatedDecl() : func)->getNumParams() - 1;
  }
  for (unsigned i = 0; i < num_params; ++i) {
    node.params.push_back(fromType(func->getParamDecl(i)->getType(), false));
  }
  if (has_pack) {
    node.params.push_back(make(Kind::kPackParams));
  }
  if (const auto *method = llvm::dyn_cast<clang::CXXMethodDecl>(func)) {
    node.is_const = method->isConst();
    node.is_volatile = method->isVolatile();
    switch (method->getRefQualifier()) {
    case clang::RQ_LValue:
      node.ref = "&";
      break;
    case clang::RQ_RValue:
      node.ref = "&&";
      break;
    default:
      break;
    }
  }
  return node;
}

std::optional<Node> Builder::FromExpr(const clang::Expr *expr) {
  expr = expr->IgnoreParenImpCasts();

  if (llvm::isa<clang::IntegerLiteral>(expr) &&
      expr->getBeginLoc().isMacroID()) {
    auto name = clang::Lexer::getImmediateMacroName(
        expr->getBeginLoc(), ctx_.getSourceManager(), ctx_.getLangOpts());
    if (!name.empty()) {
      return make(Kind::kMacro, name.str());
    }
  }

  if (const auto *call = llvm::dyn_cast<clang::CallExpr>(expr)) {
    if (const auto *callee = call->getDirectCallee()) {
      return FromDecl(callee);
    }
  }

  if (const auto *ctor = llvm::dyn_cast<clang::CXXConstructExpr>(expr)) {
    assert(ctor->getConstructor() &&
           "expr is a CXXConstructExpr but could not get constructor");
    return FromDecl(ctor->getConstructor());
  }

  if (const auto *member = llvm::dyn_cast<clang::MemberExpr>(expr)) {
    const auto *decl = member->getMemberDecl();
    if (llvm::isa<clang::CXXMethodDecl>(decl)) {
      return FromDecl(decl);
    }
    auto arrow = [&](clang::QualType object) {
      Node node = make(Kind::kArrow);
      node.object = share(FromType(object));
      node.member = share(FromDecl(decl));
      return node;
    };
    if (member->isArrow()) {
      const auto *base = member->getBase()->IgnoreParenImpCasts();
      if (const auto *op = llvm::dyn_cast<clang::CXXOperatorCallExpr>(base);
          op && op->getOperator() == clang::OO_Arrow) {
        return arrow(op->getArg(0)->IgnoreImpCasts()->getType());
      }
    } else if (auto for_range = GetParentForRange(ctx_, member)) {
      const auto *range =
          for_range->getRangeInit()->getType()->getAsCXXRecordDecl();
      if (range && llvm::isa<clang::ClassTemplateSpecializationDecl>(range) &&
          QualifiedName(range) == "std::map") {
        auto iter_type = GetForRangeIteratorType(for_range);
        if (!iter_type.isNull()) {
          return arrow(iter_type);
        }
      }
    }
    return FromDecl(decl);
  }

  if (const auto *ref = llvm::dyn_cast<clang::DeclRefExpr>(expr)) {
    return FromDecl(ref->getDecl());
  }

  if (const auto *uop = llvm::dyn_cast<clang::UnaryOperator>(expr)) {
    auto sub = FromExpr(uop->getSubExpr());
    if (!sub) {
      return std::nullopt;
    }
    Node node =
        make(Kind::kUnary,
             (uop->isPostfix() ? "post" : "") +
                 clang::UnaryOperator::getOpcodeStr(uop->getOpcode()).str());
    node.operand = share(std::move(*sub));
    return node;
  }

  return std::nullopt;
}

bool Match(const Node &rule, const Node &use, Bindings &bindings) {
  if (rule.kind == Kind::kParam) {
    if ((rule.is_const && !use.is_const) ||
        (rule.is_volatile && !use.is_volatile)) {
      return false;
    }
    Node bound = use;
    if (rule.is_const) {
      bound.is_const = false;
      bound.type.removeLocalConst();
    }
    if (rule.is_volatile) {
      bound.is_volatile = false;
      bound.type.removeLocalVolatile();
    }
    if (bindings.size() <= rule.param) {
      bindings.resize(rule.param + 1);
    }
    auto &slot = bindings[rule.param];
    if (!slot) {
      slot = std::move(bound);
      return true;
    }
    return *slot == bound;
  }
  if (rule.kind != use.kind || rule.name != use.name ||
      rule.is_const != use.is_const || rule.is_volatile != use.is_volatile ||
      rule.variadic != use.variadic || rule.ref != use.ref) {
    return false;
  }
  return Node::zipChildren(rule, use, [&](const Node &r, const Node &u) {
    return Match(r, u, bindings);
  });
}

} // namespace cpp2rust::IrSrc
