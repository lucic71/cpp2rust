// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/models/converter_refcount.h"

#include <clang/AST/RecordLayout.h>
#include <clang/Basic/OperatorKinds.h>

#include <algorithm>
#include <format>
#include <optional>
#include <vector>

#include "compiler.h"
#include "converter/converter_lib.h"
#include "converter/lex.h"
#include "converter/mapper.h"

namespace cpp2rust {
ConverterRefCount::ConverterRefCount(std::string &rs_code,
                                     clang::ASTContext &ctx)
    : Converter(rs_code, ctx, "", "", ""),
      conversion_kind_({ConversionKind::Unboxed}) {}

std::string ConverterRefCount::EmitFilePreamble() {
  return R"(
extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::{Read, Write, Seek};
use std::io::prelude::*;
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
)";
}

static bool IsBoxedType(std::string_view type) {
  return type.starts_with("Vec<") || type.starts_with("Box<");
}

static bool IsBoxedType(clang::QualType type) {
  return IsBoxedType(Mapper::Map(type.getUnqualifiedType()));
}

static bool NeedsMutAccess(const clang::CXXMethodDecl *method,
                           clang::QualType base_type) {
  return !method->isConst() && IsBoxedType(base_type);
}

static bool IsPointerType(clang::QualType type) {
  return type->isPointerType() ||
         GetStrongestIteratorCategory(type) == IteratorCategory::Contiguous;
}

bool ConverterRefCount::PointeeIsBoxed(const clang::Expr *expr) {
  if (!expr) {
    return false;
  }
  if (!IsBoxedType(expr->getType().getNonReferenceType())) {
    return false;
  }
  if (auto *ase = clang::dyn_cast<clang::ArraySubscriptExpr>(expr)) {
    auto base_type = ase->getBase()->IgnoreCasts()->getType();
    if (base_type->isPointerType()) {
      return IsBoxedType(base_type->getPointeeType());
    }
    return IsBoxedType(base_type.getNonReferenceType());
  }
  if (auto *oce = clang::dyn_cast<clang::CXXOperatorCallExpr>(expr)) {
    if (oce->getOperator() == clang::OverloadedOperatorKind::OO_Subscript) {
      return IsBoxedType(oce->getArg(0)->getType().getNonReferenceType());
    }
  }
  return false;
}

std::string ConverterRefCount::GetInnerType(clang::QualType type) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  auto str = RenderType(type);
  auto pos = str.find('<');
  auto end = str.rfind('>');
  if (str[pos + 1] == '[' && str[end - 1] == ']') {
    // Unwrap inner array type
    pos++;
    end--;
  }
  return std::move(str).substr(pos + 1, end - pos - 1);
}

ConverterRefCount::PushUnboxedIfSimple::PushUnboxedIfSimple(
    ConverterRefCount &c, std::string_view outer, clang::QualType inner_type)
    : c(c) {
  bool unboxed = outer == "Ptr<%>" || outer == "%";

  // Vectors are boxed until the last element
  if (!unboxed && (outer == "Vec<%>" || outer == "Box<%>")) {
    if (!IsBoxedType(inner_type)) {
      unboxed = true;
    }
  }

  c.conversion_kind_.push_back(unboxed ? ConversionKind::Unboxed
                                       : ConversionKind::FullRefCount);
}

std::string
ConverterRefCount::GetSafeTypeAsString(clang::QualType qual_type) const {
  std::string type_as_string;
  ConverterRefCount converter(type_as_string, ctx_);
  return std::string(Trim(converter.Convert(qual_type)->print()));
}

RsExpr *ConverterRefCount::BoxType(RsExpr *node) {
  switch (getConversionKind()) {
  case ConversionKind::Unboxed:
  case ConversionKind::UnboxedField:
  case ConversionKind::Ptr:
    return node;
  case ConversionKind::FullRefCount:
    return Cat(Text("Value<"), node, Text('>'));
  }
  std::unreachable();
}

RsExpr *ConverterRefCount::BoxValue(RsExpr *node) {
  switch (getConversionKind()) {
  case ConversionKind::Unboxed:
  case ConversionKind::UnboxedField:
  case ConversionKind::Ptr:
    return node;
  case ConversionKind::FullRefCount:
    return Cat(Text("Rc::new(RefCell::new("), node, Text("))"));
  }
  std::unreachable();
}

RsExpr *ConverterRefCount::Convert(clang::QualType qual_type) {
  // Catch va_list before desugaring
  if (IsVaListType(qual_type)) {
    return BoxType(Text("VaList"));
  }

  if (!Mapper::Contains(qual_type)) {
    qual_type = qual_type.getUnqualifiedType().getDesugaredType(ctx_);
  }

  if (qual_type->isLValueReferenceType() ||
      qual_type->isIncompleteArrayType()) {
    return Converter::Convert(qual_type);
  }

  return BoxType(Converter::Convert(qual_type));
}

RsExpr *ConverterRefCount::VisitIncompleteArrayType(
    const clang::IncompleteArrayType *type) {
  RsExpr *node = nullptr;
  {
    PushUnboxedIfSimple push(*this, "Box<%>", type->getElementType());
    node = Cat(Text("Box<["), Convert(type->getElementType()), Text("]>"));
  }
  return BoxType(node);
}

RsExpr *ConverterRefCount::VisitLValueReferenceType(
    const clang::LValueReferenceType *type) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  return Cat(Text("Ptr<"), Convert(type->getPointeeType()), Text('>'));
}

RsExpr *ConverterRefCount::BuildFnAdapter(
    const clang::FunctionDecl *src_fn,
    const clang::FunctionProtoType *src_proto,
    const clang::FunctionProtoType *target_proto) {

  // UB: Incompatible arity
  if (src_proto->getNumParams() != target_proto->getNumParams()) {
    return Text("None");
  }

  PushConversionKind push(*this, ConversionKind::Unboxed);

  // Build adapter signature: |a0: T0, a1: T1, ...| -> Tr
  std::vector<RsExpr *> parts;
  parts.push_back(Text("Some((|"));
  for (unsigned i = 0; i < target_proto->getNumParams(); ++i) {
    parts.push_back(Text(std::format("a{}:", i)));
    parts.push_back(Convert(target_proto->getParamType(i)));
    parts.push_back(Text(','));
  }
  parts.push_back(Text('|'));
  if (!target_proto->getReturnType()->isVoidType()) {
    parts.push_back(Text("->"));
    parts.push_back(Convert(target_proto->getReturnType()));
  }
  parts.push_back(Text('{'));

  // Build adapter body: src_fn(convert(a0), convert(a1), ...)
  parts.push_back(Text(Mapper::MapFunctionName(src_fn) + '('));
  for (unsigned i = 0; i < src_proto->getNumParams(); ++i) {
    auto src_pty = src_proto->getParamType(i);
    auto tgt_pty = target_proto->getParamType(i);
    if (SameRendered(Convert(src_pty), Convert(tgt_pty))) {
      parts.push_back(Text(std::format("a{}", i)));
    } else if (src_pty->isPointerType() && tgt_pty->isPointerType()) {
      if (tgt_pty->isVoidPointerType()) {
        parts.push_back(Text(std::format("a{}.reinterpret_cast::<", i)));
        parts.push_back(ConvertPointeeType(src_pty));
        parts.push_back(Text(">()"));
      } else if (src_pty->isVoidPointerType()) {
        parts.push_back(Text(std::format("a{}.to_any()", i)));
      } else if (tgt_pty->getPointeeType()->isCharType()) {
        parts.push_back(Text(std::format("a{}.reinterpret_cast::<", i)));
        parts.push_back(ConvertPointeeType(src_pty));
        parts.push_back(Text(">()"));
      } else if (src_pty->getPointeeType()->isCharType()) {
        parts.push_back(Text(std::format("a{}.reinterpret_cast::<", i)));
        parts.push_back(ConvertPointeeType(src_pty));
        parts.push_back(Text(">()"));
      }
    } else {
      // UB: Incompatible types
      return Text("None");
    }
    parts.push_back(Text(','));
  }
  if (target_proto->getReturnType()->isVoidType() &&
      !src_fn->getReturnType()->isVoidType()) {
    parts.push_back(Text("); })"));
  } else {
    parts.push_back(Text(") })"));
  }

  parts.push_back(Text("as"));
  parts.push_back(ConvertFunctionPointerType(target_proto));
  parts.push_back(Text(')'));
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *ConverterRefCount::ConvertFunctionPointerType(
    const clang::FunctionProtoType *proto, FnProtoType kind) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  return Converter::ConvertFunctionPointerType(proto, kind);
}

RsExpr *ConverterRefCount::VisitPointerType(const clang::PointerType *type) {
  if (auto proto = type->getPointeeType()->getAs<clang::FunctionProtoType>()) {
    return Cat(Text("FnPtr<"), ConvertFunctionPointerType(proto), Text('>'));
  }

  if (IsVaListType(clang::QualType(type, 0))) {
    return Text("VaList");
  }

  if (type->isVoidPointerType()) {
    return Text("AnyPtr");
  }

  auto pointee_type = type->getPointeeType();
  PushConversionKind push1(*this, ConversionKind::Ptr,
                           !pointee_type->isArrayType());
  PushConversionKind push2(*this, ConversionKind::FullRefCount,
                           pointee_type->isArrayType());
  const char *open = "Ptr<";
  if (pointee_type->isRecordType() &&
      abstract_structs_.contains(GetID(pointee_type->getAsRecordDecl()))) {
    open = "PtrDyn<dyn";
  }
  return Cat(Text(open), Convert(pointee_type), Text('>'));
}

RsExpr *ConverterRefCount::VisitRecordType(const clang::RecordType *type) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  return Converter::VisitRecordType(type);
}

RsExpr *ConverterRefCount::VisitConstantArrayType(
    const clang::ConstantArrayType *type) {
  auto conv = getConversionKind();
  PushConversionKind push(*this, ConversionKind::Unboxed);

  switch (conv) {
  case ConversionKind::Unboxed:
    return Cat(
        Text('['), Convert(type->getElementType()),
        Text(std::format("; {}]", GetNumAsString(type->getSize()).c_str())));
  case ConversionKind::Ptr:
    return Convert(type->getElementType());
  case ConversionKind::UnboxedField:
  case ConversionKind::FullRefCount:
    return Cat(Text("Box<["), Convert(type->getElementType()), Text("]>"));
  }
  std::unreachable();
}

RsExpr *ConverterRefCount::ConvertFreshLValue(clang::Expr *expr) {
  auto *node = ConvertLValue(expr);
  if (isFresh()) {
    return node;
  }
  SetFresh();
  return Cat(Text('('), node, Text(").clone()"));
}

RsExpr *ConverterRefCount::ConvertObject(clang::Expr *expr) {
  PushExprKind push(*this, ExprKind::Object);
  auto *node = ConvertExpr(expr);
  if (expr->getType()->isPointerType()) {
    computed_expr_type_ = ComputedExprType::FreshPointer;
    return Cat(node, Text(".to_strong().as_pointer()"));
  }
  return node;
}

RsExpr *ConverterRefCount::ConvertFreshObject(clang::Expr *expr) {
  auto *node = ConvertObject(expr);
  if (isFresh()) {
    return node;
  }
  SetFresh();
  return Cat(Text('('), node, Text(").clone()"));
}

RsExpr *ConverterRefCount::ConvertFresh(
    clang::Expr *expr, std::optional<clang::QualType> implicit_convert_to) {
  auto *node = ConvertExpr(expr, implicit_convert_to);
  if (isFresh() || expr->getType()->isVoidType() || isVoid()) {
    return node;
  }
  SetFresh();
  return Cat(Text('('), node, Text(").clone()"));
}

RsExpr *ConverterRefCount::ConvertFreshRValue(
    clang::Expr *expr, std::optional<clang::QualType> implicit_convert_to) {
  auto *node = ConvertRValue(expr, implicit_convert_to);
  if (!isFresh() && !expr->getType()->isVoidType()) {
    SetFresh();
    return Cat(Text('('), node, Text(").clone()"));
  }
  SetFresh();
  return node;
}

RsExpr *ConverterRefCount::ConvertFreshPointer(clang::Expr *expr) {
  auto *node = ConvertPointer(expr);
  if (isFresh()) {
    return node;
  }
  SetFresh();
  return Cat(Text('('), node, Text(").clone()"));
}

std::pair<RsExpr *, RsExpr *>
ConverterRefCount::MaterializeTemp(const std::string &binding_name,
                                   clang::QualType param_type,
                                   clang::Expr *expr) {
  auto pointee = param_type.getNonReferenceType();
  auto *value = ConvertRValue(expr, pointee);
  auto *type_node = Converter::Convert(pointee);
  auto *binding =
      Cat(Text(std::format("let {} : Value <", binding_name)), type_node,
          Text("> = Rc::new(RefCell::new("), value, Text(")) ;"));
  return {binding, Text(std::format("{}.as_pointer()", binding_name))};
}

RsExpr *ConverterRefCount::ConvertPtrType(clang::QualType type) {
  RsExpr *inner = nullptr;
  // decays into Ptr; remove the outer type Vec<>
  if (IsBoxedType(type)) {
    inner = Text(GetInnerType(type));
  } else {
    PushConversionKind push(*this, ConversionKind::Ptr);
    inner = Convert(type);
  }
  return Cat(Text("Ptr<"), inner, Text('>'));
}

RsExpr *
ConverterRefCount::VisitArraySubscriptExpr(clang::ArraySubscriptExpr *expr) {
  auto *base = expr->getBase();
  if (base->IgnoreCasts()->getType()->isPointerType() ||
      IsUnionArrayMember(base)) {
    return ConvertPointerSubscript(expr);
  }
  if (!base->IgnoreCasts()->getType()->isArrayType()) {
    if (isLValue()) {
      return arena_.New<Unary>(
          Unary::Op::Deref,
          ConvertArraySubscript(base, expr->getIdx(), expr->getType()));
    }
    auto *subscript =
        ConvertArraySubscript(base, expr->getIdx(), expr->getType());
    auto *node = arena_.New<Unary>(Unary::Op::Deref, subscript);
    SetValueFreshness(expr->getType());
    return node;
  }
  return ConvertArraySubscript(base, expr->getIdx(), expr->getType());
}

RsExpr *ConverterRefCount::VisitCXXRecordDecl(clang::CXXRecordDecl *decl) {
  if (decl_ids_.count(GetID(decl))) {
    return arena_.New<Verbatim>("");
  }
  return Converter::VisitCXXRecordDecl(decl);
}

RsExpr *ConverterRefCount::VisitOffsetOfExpr(clang::OffsetOfExpr *expr) {
  clang::Expr::EvalResult result;
  ENSURE(expr->EvaluateAsInt(result, ctx_));
  computed_expr_type_ = ComputedExprType::FreshValue;
  return Text(std::format("{}_usize", result.Val.getInt().getZExtValue()));
}

RsExpr *ConverterRefCount::ConvertOrdAndPartialOrdTraits(
    const clang::CXXRecordDecl *decl, const clang::FunctionDecl *op) {
  std::string first_branch, second_branch, first_return, second_return;

  switch (op->getOverloadedOperator()) {
  case clang::OO_Less:
    if (clang::isa<clang::CXXMethodDecl>(op)) {
      first_branch = std::format(
          "self.{}(Rc::new(RefCell::new(other.clone())).as_pointer())",
          GetOverloadedOperator(op));
      second_branch = std::format(
          "other.{}(Rc::new(RefCell::new(self.clone())).as_pointer())",
          GetOverloadedOperator(op));
    } else {
      first_branch =
          std::format("{}(Rc::new(RefCell::new(self.clone())).as_pointer(), "
                      "Rc::new(RefCell::new(other.clone())).as_pointer())",
                      GetOverloadedOperator(op));
      second_branch =
          std::format("{}(Rc::new(RefCell::new(other.clone())).as_pointer(), "
                      "Rc::new(RefCell::new(self.clone())).as_pointer())",
                      GetOverloadedOperator(op));
    }
    first_return = "std::cmp::Ordering::Less";
    second_return = "std::cmp::Ordering::Greater";
    break;
  default:
    assert(0 && "Currently only supporting operator<");
  }

  return ConvertOrdAndPartialOrdTraitsBase(first_branch, second_branch,
                                           first_return, second_return,
                                           GetRecordName(decl));
}

RsExpr *ConverterRefCount::AddCloneTrait(const clang::RecordDecl *decl) {
  auto record_name = GetRecordName(decl);

  if (decl->isUnion()) {
    return Cat(
        Text("impl Clone for"), Text(record_name),
        Braces(Cat(Text("fn clone(&self) -> Self"),
                   Braces(Cat(Text(record_name),
                              Text("{ __bytes: "
                                   "Rc::new(RefCell::new(self.__bytes.borrow()"
                                   ".clone())) }"))))));
  }

  auto *cxx = clang::dyn_cast<clang::CXXRecordDecl>(decl);
  if (!cxx) {
    return Text("");
  }

  if (cxx->defaultedCopyConstructorIsDeleted()) {
    return Text("");
  }

  RsExpr *body = Text("");
  for (auto ctor : cxx->ctors()) {
    if (ctor->isCopyConstructor()) {
      PushConversionKind push(*this, ConversionKind::UnboxedField);
      body = ConvertCXXConstructorBody(ctor);
      break;
    }
  }

  return Cat(Text(keyword::kImpl), Text("Clone for"), Text(record_name),
             Text('{'), Text("fn clone(&self) -> Self {"), body, Text('}'),
             Text('}'));
}

RsExpr *ConverterRefCount::AddDefaultTrait(const clang::RecordDecl *decl) {
  PushConversionKind push(*this, ConversionKind::UnboxedField);
  return Converter::AddDefaultTrait(decl);
}

RsExpr *
ConverterRefCount::AddDefaultTraitForUnion(const clang::RecordDecl *decl) {
  auto name = GetRecordName(decl);
  return Cat(
      Text("impl Default for"), Text(name),
      Braces(Cat(
          Text("fn default() -> Self"),
          Braces(Text(std::format(
              "{} {{ __bytes: Rc::new(RefCell::new(Box::from([0u8; {}]))) }}",
              name, ctx_.getASTRecordLayout(decl).getSize().getQuantity()))))));
}

RsExpr *ConverterRefCount::EmitRustUnion(clang::RecordDecl *decl) {
  auto name = GetRecordName(decl);

  auto attrs = GetStructAttributes(decl);
  Mapper::SetDerives(ctx_.getCanonicalTagType(decl),
                     std::vector<std::string>(attrs.begin(), attrs.end()));

  std::vector<RsExpr *> parts;
  parts.push_back(
      Text(std::format("pub struct {} {{ __bytes: Value<Box<[u8]>> }}", name)));

  std::vector<RsExpr *> accessors;
  for (auto *field : decl->fields()) {
    PushConversionKind push(*this, ConversionKind::Unboxed);
    auto *ty =
        field->getType()->isArrayType()
            ? Convert(
                  field->getType()->getAsArrayTypeUnsafe()->getElementType())
            : Convert(field->getType());
    accessors.push_back(Cat(Text(std::format("pub fn {}(&self) -> Ptr<",
                                             GetNamedDeclAsString(field))),
                            ty,
                            Text("> { (self.__bytes.as_pointer() "
                                 "as Ptr<u8>).reinterpret_cast() }")));
  }
  parts.push_back(Text("impl"));
  parts.push_back(Text(name));
  parts.push_back(Braces(arena_.New<Concat>(std::move(accessors))));

  parts.push_back(AddCloneTrait(decl));
  parts.push_back(AddDefaultTrait(decl));
  parts.push_back(AddByteReprTrait(decl));
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *ConverterRefCount::AddDropTrait(const clang::CXXRecordDecl *decl) {
  if (!decl->hasUserDeclaredDestructor()) {
    return Text("");
  }

  auto dtor = decl->getDestructor();
  if (!dtor) {
    return Text("");
  }

  auto body = dtor->getBody();
  if (!body) {
    return Text("");
  }

  if (auto stmt = llvm::dyn_cast<clang::CompoundStmt>(body)) {
    if (stmt->body_empty()) {
      return Text("");
    }
  }

  auto record_name = GetRecordName(decl);
  auto *body_node = ConvertFullStmt(body);

  return Cat(Text(keyword::kImpl), Text("Drop for"), Text(record_name),
             Text('{'), Text("fn drop(&mut self) {"), body_node, Text('}'),
             Text('}'));
}

RsExpr *
ConverterRefCount::EmitBitFieldToBytes(const clang::FieldDecl *field,
                                       const clang::ASTRecordLayout &layout) {
  assert(!field->isUnnamedBitField());
  auto bit_off = layout.getFieldOffset(field->getFieldIndex());
  auto width = field->getBitWidthValue();

  std::string stmts =
      std::format("{{ let __v = self.{} as u64;", GetNamedDeclAsString(field));
  for (auto byte = bit_off / 8; byte <= (bit_off + width - 1) / 8; ++byte) {
    auto lo = std::max(bit_off, byte * 8);
    auto hi = std::min(bit_off + width, byte * 8 + 8);
    auto mask = ((1U << (hi - lo)) - 1) << (lo - byte * 8);
    stmts += std::format(
        "buf[{0}] = (buf[{0}] & !{1:#04x}u8) | ((((__v >> {2}) as u8) << {3}) "
        "& {1:#04x}u8);",
        byte, mask, lo - bit_off, lo - byte * 8);
  }
  return Text(stmts + '}');
}

RsExpr *
ConverterRefCount::EmitBitFieldFromBytes(const clang::FieldDecl *field,
                                         const clang::ASTRecordLayout &layout,
                                         const std::string &storage_ty) {
  assert(!field->isUnnamedBitField());
  auto bit_off = layout.getFieldOffset(field->getFieldIndex());
  auto width = field->getBitWidthValue();

  std::string raw;
  for (auto byte = bit_off / 8; byte <= (bit_off + width - 1) / 8; ++byte) {
    auto lo = std::max(bit_off, byte * 8);
    auto hi = std::min(bit_off + width, byte * 8 + 8);
    if (!raw.empty()) {
      raw += " | ";
    }
    raw += std::format("(((buf[{}] as u64 >> {}) & {:#x}) << {})", byte,
                       lo - byte * 8, (1U << (hi - lo)) - 1, lo - bit_off);
  }

  std::string value;
  if (field->getType()->isSignedIntegerType()) {
    value = std::format("(((({0}) << {1}) as i64) >> {1}) as {2}", raw,
                        64 - width, storage_ty);
  } else {
    assert(field->getType()->isUnsignedIntegerType());
    value = std::format("({}) as {}", raw, storage_ty);
  }

  return Text(std::format("{}: {},", GetNamedDeclAsString(field), value));
}

RsExpr *ConverterRefCount::AddByteReprTrait(const clang::RecordDecl *decl) {
  auto struct_name = GetRecordName(decl);

  if (!TypeImplementsByteRepr(ctx_.getCanonicalTagType(decl))) {
    return Cat(Text(std::format("impl ByteRepr for {}", struct_name)),
               Braces(Text("")));
  }

  std::vector<RsExpr *> body;

  if (decl->isUnion()) {
    body.push_back(Text(
        std::format("fn byte_size() -> usize {{ {} }}",
                    ctx_.getTypeSize(ctx_.getCanonicalTagType(decl)) / 8)));
    body.push_back(Text("fn to_bytes(&self, buf: &mut [u8]) { "
                        "buf.copy_from_slice(&self.__bytes.borrow()); }"));
    body.push_back(
        Text(std::format("fn from_bytes(buf: &[u8]) -> Self {{ {} {{ __bytes: "
                         "Rc::new(RefCell::new(Box::from(buf))) }} }}",
                         struct_name)));
    return Cat(Text("impl ByteRepr for "), Text(struct_name),
               Braces(arena_.New<Concat>(std::move(body))));
  }

  const auto &layout = ctx_.getASTRecordLayout(decl);

  body.push_back(
      Text(std::format("fn byte_size() -> usize {{ {} }}",
                       ctx_.getTypeSize(ctx_.getCanonicalTagType(decl)) / 8)));

  std::vector<RsExpr *> to_bytes;
  for (auto *field : decl->fields()) {
    if (field->isBitField()) {
      to_bytes.push_back(EmitBitFieldToBytes(field, layout));
      continue;
    }
    auto byte_off = layout.getFieldOffset(field->getFieldIndex()) / 8;
    auto byte_size = ctx_.getTypeSize(field->getType()) / 8;
    to_bytes.push_back(Text(std::format("self.{}.to_bytes(&mut buf[{}..{}]);",
                                        GetNamedDeclAsString(field), byte_off,
                                        byte_off + byte_size)));
  }
  body.push_back(Text("fn to_bytes(&self, buf: &mut [u8])"));
  body.push_back(Braces(arena_.New<Concat>(std::move(to_bytes))));

  std::vector<RsExpr *> from_bytes;
  for (auto *field : decl->fields()) {
    PushConversionKind push(*this, ConversionKind::UnboxedField);
    std::string storage_ty = RenderType(field->getType());
    if (field->isBitField()) {
      from_bytes.push_back(EmitBitFieldFromBytes(field, layout, storage_ty));
      continue;
    }
    auto byte_off = layout.getFieldOffset(field->getFieldIndex()) / 8;
    auto byte_size = ctx_.getTypeSize(field->getType()) / 8;
    from_bytes.push_back(Text(std::format(
        "{}: <{}>::from_bytes(&buf[{}..{}]),", GetNamedDeclAsString(field),
        storage_ty, byte_off, byte_off + byte_size)));
  }
  body.push_back(Text("fn from_bytes(buf: &[u8]) -> Self"));
  body.push_back(Braces(
      Cat(Text("Self"), Braces(arena_.New<Concat>(std::move(from_bytes))))));

  return Cat(Text("impl ByteRepr for "), Text(struct_name),
             Braces(arena_.New<Concat>(std::move(body))));
}

RsExpr *ConverterRefCount::AddByteReprTrait(const clang::EnumDecl *decl) {
  auto name = GetRecordName(decl);
  auto byte_size = ctx_.getTypeSize(decl->getIntegerType()) / 8;
  return Cat(
      Text(std::format("impl ByteRepr for {}", name)),
      Braces(
          Cat(Text(std::format("fn byte_size() -> usize {{ {} }}", byte_size)),
              Text("fn to_bytes(&self, buf: &mut [u8]) { (*self as i32)"
                   ".to_bytes(buf); }"),
              Text(std::format("fn from_bytes(buf: &[u8]) -> Self {{ "
                               "<{}>::from(i32::from_bytes(buf)) }}",
                               name)))));
}

bool ConverterRefCount::IsMethodOnPtr(clang::CXXMethodDecl *method) {
  return IsTranslatableMethod(method) && !method->isStatic() &&
         !clang::isa<clang::CXXConstructorDecl>(method) &&
         !method->isOverloadedOperator();
}

bool ConverterRefCount::MethodHasVisibility(clang::CXXMethodDecl *decl) {
  return !IsMethodOnPtr(decl);
}

RsExpr *ConverterRefCount::EmitOutOfLineMethod(clang::CXXMethodDecl *decl,
                                               RsExpr *inner) {
  if (!IsMethodOnPtr(decl)) {
    return Converter::EmitOutOfLineMethod(decl, inner);
  }
  auto *record = decl->getParent();
  return arena_.New<Impl>(
      std::vector<RsExpr *>{}, std::format("{}Methods", GetRecordName(record)),
      Convert(ctx_.getPointerType(ctx_.getCanonicalTagType(record))),
      std::vector<RsExpr *>{inner});
}

bool ConverterRefCount::ThisIsValue() const {
  auto *method = clang::dyn_cast_or_null<clang::CXXMethodDecl>(curr_function_);
  return !method || !IsMethodOnPtr(method);
}

RsExpr *ConverterRefCount::ConvertRecordMethods(clang::CXXRecordDecl *decl) {
  std::vector<RsExpr *> parts;
  auto struct_name = GetRecordName(decl);

  auto record_methods = CollectCXXMethodDecls(decl, [](auto *method) {
    return IsMethodOnRecord(method) && !IsMethodOnPtr(method);
  });
  if (!record_methods.empty()) {
    parts.push_back(arena_.New<Impl>(std::vector<RsExpr *>{}, "",
                                     Text(struct_name),
                                     std::move(record_methods)));
  }

  std::vector<clang::CXXMethodDecl *> ptr_methods;
  for (auto *method : decl->methods()) {
    if (IsMethodOnPtr(method)) {
      ptr_methods.push_back(method);
    }
  }

  if (!ptr_methods.empty()) {
    auto trait_name = std::format("{}Methods", struct_name);

    std::vector<RsExpr *> declarations;
    std::vector<RsExpr *> definitions;
    for (auto *method : ptr_methods) {
      declarations.push_back(ConvertMethodItem(method, false, false));
      if (method->isThisDeclarationADefinition()) {
        definitions.push_back(ConvertMethodItem(method, false, true));
      }
    }

    parts.push_back(
        arena_.New<Trait>(std::vector<RsExpr *>{Text(keyword::kPub)},
                          trait_name, std::move(declarations)));
    if (!definitions.empty()) {
      parts.push_back(arena_.New<Impl>(
          std::vector<RsExpr *>{}, trait_name,
          Convert(ctx_.getPointerType(ctx_.getCanonicalTagType(decl))),
          std::move(definitions)));
    }
  }

  parts.push_back(ConvertVirtualMethods(decl));
  return arena_.New<Concat>(std::move(parts));
}

Fn::Receiver
ConverterRefCount::GetMethodReceiver(const clang::CXXMethodDecl *decl) {
  return Fn::Receiver::Ref;
}

RsExpr *
ConverterRefCount::VisitCXXConstructorDecl(clang::CXXConstructorDecl *decl) {
  PushConversionKind push(*this, ConversionKind::UnboxedField);
  return Converter::VisitCXXConstructorDecl(decl);
}

RsExpr *ConverterRefCount::VisitFieldDecl(clang::FieldDecl *decl) {
  PushConversionKind push(*this, ConversionKind::UnboxedField);
  return Converter::VisitFieldDecl(decl);
}

RsExpr *ConverterRefCount::EmitFunctionPreamble(clang::FunctionDecl *decl) {
  // In the header, the function might be declared as `int foo(int name_1)',
  // while in the source file the function might be defined as `int foo(int
  // name_2)'. We want to get the parameters from the definition if possible,
  // i.e. name_2.
  PushConversionKind push(*this, ConversionKind::FullRefCount);
  auto params = decl->getDefinition() ? decl->getDefinition()->parameters()
                                      : decl->parameters();
  std::vector<RsExpr *> parts;
  for (auto *param : params) {
    if (!param->getType()->isReferenceType()) {
      auto name = GetNamedDeclAsString(param);
      // Skip emitting the preamble for unnamed parameters
      if (name == "_") {
        continue;
      }

      auto *type = Convert(param->getType());

      if (param->hasDefaultArg()) {
        auto *default_arg = ConvertExpr(param->getDefaultArg());
        parts.push_back(Text(std::format("let {} :", name)));
        parts.push_back(type);
        parts.push_back(
            Text(std::format("= Rc::new(RefCell::new({}.unwrap_or(", name)));
        parts.push_back(default_arg);
        parts.push_back(Text(")))"));
      } else {
        parts.push_back(Text(std::format("let {} :", name)));
        parts.push_back(type);
        parts.push_back(Text(std::format("= Rc::new(RefCell::new({}))", name)));
      }
      parts.push_back(Text(token::kSemiColon));
    }
  }
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *ConverterRefCount::ConvertVaListVarDecl(clang::VarDecl *decl) {
  std::vector<RsExpr *> parts;
  if (clang::isa<clang::ParmVarDecl>(decl)) {
    // va_list parameter (decayed to __va_list_tag *)
  } else {
    // va_list local variable
    parts.push_back(Text(keyword::kLet));
  }

  parts.push_back(Text(GetNamedDeclAsString(decl)));
  parts.push_back(Text(token::kColon));
  parts.push_back(Text("Value<VaList>"));
  return arena_.New<Concat>(std::move(parts));
}

bool ConverterRefCount::ConvertLambdaVarDecl(clang::VarDecl *decl) {
  return false;
}

std::pair<RsExpr *, bool>
ConverterRefCount::ConvertVarDeclSkipInit(clang::VarDecl *decl) {
  bool unboxed = in_function_formals_;
  PushConversionKind push(*this, unboxed ? ConversionKind::Unboxed
                                         : ConversionKind::FullRefCount);
  return Converter::ConvertVarDeclSkipInit(decl);
}

RsExpr *ConverterRefCount::EmitHoistedInArmAssignment(clang::VarDecl *decl) {
  if (!decl->hasInit()) {
    return Text("");
  }
  PushConversionKind push(*this, ConversionKind::FullRefCount);
  auto *init = ConvertVarInitValue(decl->getType(), decl->getInit());
  return Cat(Text(token::kStar), Text(GetNamedDeclAsString(decl)),
             Text(".borrow_mut()"), Text(token::kAssign), init,
             Text(token::kSemiColon));
}

RsExpr *ConverterRefCount::ConvertGlobalVarDecl(clang::VarDecl *decl) {
  auto *decl_node = ConvertVarDecl(decl);
  return Cat(Text("thread_local!"), Parens(decl_node), Text(token::kSemiColon));
}

RsExpr *ConverterRefCount::VisitVarDecl(clang::VarDecl *decl) {
  bool unboxed = in_function_formals_;
  PushConversionKind push(*this, unboxed ? ConversionKind::Unboxed
                                         : ConversionKind::FullRefCount);
  if (decl->getType()->isReferenceType()) {
    PushExprKind push(*this, ExprKind::AddrOf);
    return Converter::VisitVarDecl(decl);
  }
  return Converter::VisitVarDecl(decl);
}

RsExpr *ConverterRefCount::ConvertIncAndDec(clang::UnaryOperator *expr) {
  auto opcode = expr->getOpcode();
  auto *sub_expr = expr->getSubExpr();

  const char *method = nullptr;
  switch (opcode) {
  case clang::UO_PostInc:
    method = "postfix_inc";
    break;
  case clang::UO_PostDec:
    method = "postfix_dec";
    break;
  case clang::UO_PreInc:
    method = "prefix_inc";
    break;
  case clang::UO_PreDec:
    method = "prefix_dec";
    break;
  default:
    return nullptr;
  }

  auto *node = ConvertLValue(sub_expr);
  auto *result =
      MethodCall(node, method, std::vector<RsExpr *>{}, /*is_mut=*/true);
  SetFreshType(expr->getType());
  return result;
}

RsExpr *ConverterRefCount::LowerPtrUse(RsExpr *node) {
  if (clang::isa<Cast>(node) || clang::isa<Delim>(node)) {
    return nullptr;
  }

  if (auto *assign = clang::dyn_cast<Assign>(node)) {
    if (auto *ptr = assign->left->Pointer()) {
      return arena_.New<PtrWrite>(ptr, assign->right);
    }
    if (auto *ptr = assign->left->TakePtr(Text("__v"))) {
      return arena_.New<PtrWith>(ptr, true,
                                 arena_.New<Closure>("__v", nullptr, node));
    }
    return nullptr;
  }

  if (auto *assign = clang::dyn_cast<CompoundAssign>(node)) {
    if (auto *ptr = assign->left->Pointer()) {
      std::string_view op = assign->op;
      op.remove_suffix(1); // remove '='
      auto *value = Cat(arena_.New<PtrRead>(Text("_ptr")),
                        Text(std::string(op)), assign->right);
      return Braces(Cat(Text(keyword::kLet), Text("_ptr"), Text(token::kAssign),
                        ptr, Text(".clone()"), Text(token::kSemiColon),
                        arena_.New<PtrWrite>(Text("_ptr"), value)));
    }
    if (auto *ptr = assign->left->TakePtr(Text("__v"))) {
      return arena_.New<PtrWith>(ptr, true,
                                 arena_.New<Closure>("__v", nullptr, node));
    }
    return nullptr;
  }

  if (auto *call = clang::dyn_cast<Call>(node); call && call->is_mut) {
    if (auto *ptr = call->TakePtr(Text("__v"))) {
      return arena_.New<PtrWith>(ptr, true,
                                 arena_.New<Closure>("__v", nullptr, node));
    }
    return nullptr;
  }

  if (auto *ptr = node->TakePtr(Text("(*__v)"))) {
    auto *body = node;
    if (!node->expr || !ExprIsCopyable(node->expr)) {
      body = MethodCall(body, "clone", std::vector<RsExpr *>{},
                        /*is_mut=*/false);
    }
    return arena_.New<PtrWith>(ptr, false,
                               arena_.New<Closure>("__v", nullptr, body));
  }

  if (auto *unary = clang::dyn_cast<Unary>(node);
      unary && unary->op == Unary::Op::Deref) {
    return arena_.New<PtrRead>(unary->operand);
  }

  return nullptr;
}

static bool MergeSameObjectWith(RsExpr *&slot, const std::string &object,
                                bool &is_mut) {
  if (auto *with = clang::dyn_cast<PtrWith>(slot)) {
    if (with->object->print() != object) {
      return MergeSameObjectWith(with->object, object, is_mut);
    }
    is_mut = is_mut || with->is_mut;
    slot = clang::cast<Closure>(with->closure)->body;
    MergeSameObjectWith(slot, object, is_mut);
    return true;
  }

  bool merged = false;
  slot->ForEachChild([&](RsExpr *&child) {
    merged |= MergeSameObjectWith(child, object, is_mut);
  });
  return merged;
}

RsExpr *ConverterRefCount::NestPtrUse(RsExpr *node) {
  if (!clang::isa<PtrWith>(node)) {
    return nullptr;
  }
  auto *with = clang::cast<PtrWith>(node);
  auto *body = clang::cast<Closure>(with->closure);
  bool is_mut = with->is_mut;
  if (MergeSameObjectWith(body->body, with->object->print(), is_mut)) {
    with->is_mut = is_mut;
  }

  auto *outer = node;
  while (auto *inner = outer->TakeWith()) {
    outer = arena_.New<PtrWith>(inner->object, inner->is_mut,
                                arena_.New<Closure>("__v", nullptr, outer));
  }
  if (outer == node) {
    return nullptr;
  }
  auto *closure = clang::cast<Closure>(clang::cast<PtrWith>(outer)->closure);
  if (auto *nested = NestPtrUse(closure->body)) {
    closure->body = nested;
  }
  return outer;
}

static bool MayReachOtherBorrow(RsExpr *node) {
  if (clang::isa<PtrWith>(node) || clang::isa<PtrRead>(node) ||
      clang::isa<PtrWrite>(node) || clang::isa<Call>(node) ||
      clang::isa<BorrowRead>(node) || clang::isa<BorrowWrite>(node)) {
    return true;
  }
  bool found = false;
  node->ForEachChild(
      [&](RsExpr *&child) { found = found || MayReachOtherBorrow(child); });
  return found;
}

static bool UsesClosureParam(RsExpr *node, const std::string &param) {
  if (auto *closure = clang::dyn_cast<Closure>(node);
      closure && closure->param == param) {
    return false;
  }
  if (auto *verbatim = clang::dyn_cast<Verbatim>(node)) {
    return verbatim->text.find(param) != std::string::npos;
  }
  bool used = false;
  node->ForEachChild(
      [&](RsExpr *&child) { used = used || UsesClosureParam(child, param); });
  return used;
}

RsExpr *ConverterRefCount::HoistBorrowedObject(Accessor *acc) {
  auto *object = acc->object->IgnoreParens();
  if (!clang::isa<Field>(object) && !clang::isa<Index>(object)) {
    return nullptr;
  }
  if (!object->ContainsBorrow()) {
    return nullptr;
  }
  auto *hoisted = acc->object;
  acc->object = Text("__ptr");
  return Cat(Text(keyword::kLet), Text("__ptr"), Text(token::kAssign), hoisted,
             Text(".clone()"), Text(token::kSemiColon));
}

RsExpr *ConverterRefCount::HoistPtrWrite(PtrWrite *write) {
  auto *obj_let = HoistBorrowedObject(write);
  bool hoist_value = MayReachOtherBorrow(write->value);
  if (!obj_let && !hoist_value) {
    return nullptr;
  }
  std::vector<RsExpr *> parts;
  if (obj_let) {
    parts.push_back(obj_let);
  }
  if (hoist_value) {
    auto *value = write->value;
    write->value = Text("__rhs");
    parts.push_back(Cat(Text(keyword::kLet), Text("__rhs"),
                        Text(token::kAssign), value, Text(token::kSemiColon)));
  }
  parts.push_back(write);
  return Braces(arena_.New<Concat>(std::move(parts)));
}

RsExpr *ConverterRefCount::HoistPtrUse(RsExpr *node) {
  if (auto *write = clang::dyn_cast<PtrWrite>(node)) {
    return HoistPtrWrite(write);
  }
  auto *with = clang::dyn_cast<PtrWith>(node);
  if (!with) {
    return nullptr;
  }
  auto *closure = clang::dyn_cast<Closure>(with->closure);
  if (!closure) {
    return nullptr;
  }
  if (auto *inner = HoistPtrUse(closure->body)) {
    closure->body = inner;
    return nullptr;
  }
  if (auto *body_with = clang::dyn_cast<PtrWith>(closure->body)) {
    if (body_with->is_mut &&
        UsesClosureParam(body_with->object, closure->param) &&
        !UsesClosureParam(body_with->closure, closure->param)) {
      auto *obj_read = arena_.New<PtrWith>(
          with->object, with->is_mut,
          arena_.New<Closure>(closure->param, nullptr, body_with->object));
      return Braces(
          Cat(Text(keyword::kLet), Text("__obj"), Text(token::kAssign),
              obj_read, Text(token::kSemiColon),
              arena_.New<PtrWith>(Text("__obj"), true, body_with->closure)));
    }
  }
  if (!with->is_mut) {
    return nullptr;
  }
  RsExpr **right = nullptr;
  if (auto *assign = clang::dyn_cast<Assign>(closure->body)) {
    right = &assign->right;
  } else if (auto *assign = clang::dyn_cast<CompoundAssign>(closure->body)) {
    right = &assign->right;
  }
  bool hoist_rhs = right && MayReachOtherBorrow(*right) &&
                   !UsesClosureParam(*right, closure->param);
  auto *obj_let = with->is_mut ? HoistBorrowedObject(with) : nullptr;
  if (!hoist_rhs && !obj_let) {
    return nullptr;
  }
  std::vector<RsExpr *> parts;
  if (obj_let) {
    parts.push_back(obj_let);
  }
  if (hoist_rhs) {
    auto *rhs = *right;
    *right = Text("__rhs");
    parts.push_back(Cat(Text(keyword::kLet), Text("__rhs"),
                        Text(token::kAssign), rhs, Text(token::kSemiColon)));
  }
  parts.push_back(node);
  return Braces(arena_.New<Concat>(std::move(parts)));
}

RsExpr *
ConverterRefCount::VisitConditionalOperator(clang::ConditionalOperator *expr) {
  auto *cond = ConvertCondition(expr->getCond());
  auto *then_node = ConvertFresh(expr->getTrueExpr(), expr->getType());
  auto *else_node = ConvertFresh(expr->getFalseExpr(), expr->getType());
  return Cat(Text(keyword::kIf), cond, Braces(then_node), Text(keyword::kElse),
             Braces(else_node));
}

RsExpr *ConverterRefCount::VisitDeclRefExpr(clang::DeclRefExpr *expr) {
  if (isAddrOf()) {
    clang::Expr *addrof_op = ToAddrOf(ctx_, expr);
    if (auto *mapped = GetMappedAsNode(addrof_op)) {
      return mapped;
    }
  }

  if (ShouldReplaceWithMappedBody(expr)) {
    if (auto *mapped = GetMappedAsNode(expr)) {
      return mapped;
    }
  }

  auto *node = ConvertDeclRefExpr(expr);
  auto decl = expr->getDecl();

  if (auto fn_decl = clang::dyn_cast<clang::FunctionDecl>(decl)) {
    if (isAddrOf()) {
      return ConvertFunctionToFunctionPointer(fn_decl);
    }
    return node;
  }

  if (clang::isa<clang::EnumConstantDecl>(decl)) {
    return node;
  }

  auto *name = clang::dyn_cast<Verbatim>(node);
  bool is_self = name && name->text == "self";

  if (IsGlobalVar(expr)) {
    node = Cat(node, Text(".with(Value::clone)"));
  }

  if (auto *ref = decl->getType()->getAs<clang::ReferenceType>()) {
    if (map_iter_decls_.contains(clang::dyn_cast<clang::VarDecl>(decl))) {
      return node;
    }

    // std::vector<T>& gets converted to Ptr<vec<T>>
    // So we need to make a pointer to the vector itself
    if (isObject()) {
      if (IsBoxedType(ref->getPointeeType())) {
        computed_expr_type_ = ComputedExprType::FreshPointer;
        return Cat(node, Text(".to_strong().as_pointer()"));
      }
    }

    // references are not boxed
    if (isAddrOf()) {
      computed_expr_type_ = ComputedExprType::Pointer;
      return node;
    }
    if (is_self) {
      SetValueFreshness(expr->getType());
      return node;
    }
    if (isLValue()) {
      return arena_.New<Unary>(Unary::Op::Deref, node);
    }
    auto *deref = arena_.New<Unary>(Unary::Op::Deref, node);
    SetValueFreshness(expr->getType());
    return deref;
  }

  if (isAddrOf()) {
    computed_expr_type_ = ComputedExprType::FreshPointer;
    return Cat(node, Text(".as_pointer()"));
  }

  node = isRValue() ? static_cast<RsExpr *>(arena_.New<BorrowRead>(node))
                    : arena_.New<BorrowWrite>(node);

  if (auto *var_decl = clang::dyn_cast<clang::VarDecl>(expr->getDecl())) {
    if (var_decl->getType()->isPointerType()) {
      computed_expr_type_ = ComputedExprType::Pointer;
      return node;
    }
  }
  SetValueFreshness(expr->getType());
  return node;
}

static std::vector<const char *> printf2fmt(std::string &format) {
  std::vector<const char *> types;
  size_t pos = 0;
  while ((pos = format.find('%', pos)) != std::string::npos) {
    if (pos + 1 >= format.size())
      break;

    switch (auto c = format[pos + 1]) {
    case 'c':
      types.emplace_back("u8 as char");
      format.replace(pos, 2, "{}");
      pos += 2;
      continue;
    case 'd':
    case 'i':
    case 's':
    case 'u':
      types.emplace_back();
      format.replace(pos, 2, "{}");
      pos += 2;
      continue;
    case 'x':
      types.emplace_back();
      format.replace(pos, 2, "{:x}");
      pos += 4;
      continue;
    case 'p':
      types.emplace_back();
      format.replace(pos, 2, "{:?}");
      pos += 2;
      continue;
    case '%':
      types.emplace_back();
      format.replace(pos, 2, "%");
      pos += 2;
      continue;
    case 'l':
      if (pos + 2 < format.size() &&
          (format[pos + 2] == 'd' || format[pos + 2] == 'u')) {
        types.emplace_back();
        format.replace(pos, 3, "{}");
        pos += 2;
        continue;
      }
      if (pos + 3 < format.size() && format[pos + 2] == 'l' &&
          (format[pos + 3] == 'd' || format[pos + 3] == 'u')) {
        types.emplace_back();
        format.replace(pos, 4, "{}");
        pos += 2;
        continue;
      }
      break;
    case 'z':
      if (pos + 2 < format.size() &&
          (format[pos + 2] == 'd' || format[pos + 2] == 'u')) {
        types.emplace_back();
        format.replace(pos, 3, "{}");
        pos += 2;
        continue;
      }
      break;
    case '.':
      if (pos + 3 < format.size() && format[pos + 2] == '0') {
        auto end = format.find_first_not_of("0123456789", pos + 3);
        if (end != std::string::npos && format[end] == 'f') {
          auto repl = "{:." + format.substr(pos + 3, end - pos - 3) + '}';
          format.replace(pos, end - pos + 1, repl);
          pos += repl.size();
          types.emplace_back();
          continue;
        }
      }
      break;
    default:
      if (c >= '0' && c <= '9') {
        auto end = format.find_first_not_of("0123456789", pos + 2);
        if (end != std::string::npos) {
          auto repl = "{:" + format.substr(pos + 1, end - pos - 1);
          bool ok = true;
          switch (c = format[end]) {
          case 'd':
            break;
          case 'x':
            repl += c;
            break;
          case 'z':
            if (end + 1 < format.size() && format[end + 1] == 'u') {
              ++end;
            } else {
              ok = false;
            }
            break;
          case '.': {
            auto prec_end = format.find_first_not_of("0123456789", end + 1);
            if (prec_end == std::string::npos || format[prec_end] != 'f') {
              ok = false;
              break;
            }
            auto width = format.substr(pos + 1, end - pos - 1);
            repl = "{:" + (width == "0" ? "" : width) + "." +
                   format.substr(end + 1, prec_end - end - 1);
            end = prec_end;
            break;
          }
          default:
            ok = false;
            break;
          }
          if (ok) {
            repl += '}';
            format.replace(pos, end - pos + 1, repl);
            pos += repl.size();
            types.emplace_back();
            continue;
          }
        }
      }
    }
    llvm::errs() << "Unknown printf format: " << format << '\n';
    assert(0);
  }
  return types;
}

RsExpr *ConverterRefCount::ConvertPrintf(clang::CallExpr *expr) {
  bool is_fprintf =
      Mapper::ToString(expr->getCallee()).starts_with("int fprintf");
  std::string format;
  if (auto *str = clang::dyn_cast<clang::StringLiteral>(
          expr->getArg(is_fprintf)->IgnoreImplicit())) {
    format = GetEscapedStringLiteral(str);
  } else {
    return nullptr;
  }
  bool ends_newline = format.ends_with("\\n\"");

  const char *macro = nullptr;
  auto fd = is_fprintf ? Mapper::ToString(expr->getArg(0)) : "stdout";
  if (fd == "stdout" || fd == "__stdoutp") {
    macro = ends_newline ? "println!(" : "print!(";
  } else if (fd == "stderr" || fd == "__stderrp") {
    macro = ends_newline ? "eprintln!(" : "eprint!(";
  } else {
    return nullptr;
  }
  if (ends_newline) {
    format.replace(format.size() - 3, 2, "");
  }
  auto types = printf2fmt(format);

  std::vector<RsExpr *> parts;
  parts.push_back(Text(macro));
  parts.push_back(Text(std::move(format)));

  unsigned j = 0;
  for (unsigned i = is_fprintf + 1, e = expr->getNumArgs(); i < e; ++i) {
    parts.push_back(Text(token::kComma));
    auto *arg = ConvertExpr(expr->getArg(i));
    if (types[j]) {
      arg = arena_.New<Cast>(arg, Text(types[j++]));
    }
    parts.push_back(arg);
  }
  parts.push_back(Text(')'));
  return arena_.New<Concat>(std::move(parts));
}

RsExpr *ConverterRefCount::VisitCallExpr(clang::CallExpr *expr) {
  if (IsBuiltinVaStart(expr) || IsBuiltinVaEnd(expr) || IsBuiltinVaCopy(expr)) {
    return ConvertVAArgCall(expr);
  }

  if (expr->isCallToStdMove()) {
    RsExpr *node = nullptr;
    if (IsUniquePtr(expr->getArg(0)->getType())) {
      node = Cat(ConvertLValue(expr->getArg(0)), Text(".take()"));
    } else {
      PushExprKind push(*this, ExprKind::XValue);
      node = ConvertExpr(expr->getArg(0));
    }
    computed_expr_type_ = ComputedExprType::FreshValue;
    return node;
  }

  if (auto *opcall = clang::dyn_cast<clang::CXXOperatorCallExpr>(expr);
      opcall && !Mapper::Contains(expr->getCallee())) {
    return ConvertCXXOperatorCallExpr(opcall);
  }

  if (auto *plugin_node = TryPluginConvert(expr)) {
    return plugin_node;
  }

  std::optional<TempMaterializationCtx> ctx;
  RsExpr *call = nullptr;
  {
    PushConversionKind push(*this, ConversionKind::Unboxed);
    auto [node, call_ctx] = Converter::ConvertCallExpr(expr);
    call = node;
    ctx = std::move(call_ctx);
  }

  auto wrap_bindings = [&](RsExpr *node) -> RsExpr * {
    if (ctx && !ctx->temporary_bindings.empty()) {
      std::vector<RsExpr *> parts = ctx->temporary_bindings;
      parts.push_back(node);
      return Braces(arena_.New<Concat>(std::move(parts)));
    }
    return node;
  };

  auto ty = GetReturnTypeOfFunction(expr);
  auto ref = clang::dyn_cast<clang::ReferenceType>(ty);

  if (ref && !isAddrOf() && !isVoid()) {
    if (isLValue()) {
      return arena_.New<Unary>(Unary::Op::Deref, wrap_bindings(call));
    }
    // Apply deref before block wrapping so temporaries are still alive.
    auto *node = wrap_bindings(arena_.New<Unary>(Unary::Op::Deref, call));
    SetValueFreshness(ref->getPointeeType());
    return node;
  }

  if (isAddrOf() && !ty->isReferenceType() && !IsPointerType(ty)) {
    PushConversionKind push(*this, ConversionKind::FullRefCount);
    return Cat(BoxValue(call), Text(".as_pointer()"));
  }

  if (isObject()) {
    return Cat(call, Text(".to_strong().as_pointer()"));
  }

  auto *node = wrap_bindings(call);
  if (ctx && !ctx->temporary_bindings.empty()) {
    node = Parens(node);
  }
  if (IsPointerType(ty) || ty->isReferenceType()) {
    computed_expr_type_ = ComputedExprType::FreshPointer;
  } else {
    computed_expr_type_ = ComputedExprType::FreshValue;
  }
  return node;
}

RsExpr *ConverterRefCount::VisitStringLiteral(clang::StringLiteral *expr) {
  computed_expr_type_ = ComputedExprType::FreshValue;
  if (!curr_init_type_.empty() && curr_init_type_.back()->isArrayType()) {
    uint64_t pad = 1;
    if (auto *arr_ty = ctx_.getAsConstantArrayType(curr_init_type_.back())) {
      uint64_t arr_size = arr_ty->getSize().getZExtValue();
      if (expr->getString().empty()) {
        return Text(std::format("vec![0u8; {}].into_boxed_slice()", arr_size));
      }
      pad = arr_size > expr->getString().size()
                ? arr_size - expr->getString().size()
                : 0;
    }
    return Text(
        std::format("Box::from(*b{})", GetEscapedStringLiteral(expr, pad)));
  }
  return Text(std::format("b{}", GetEscapedStringLiteral(expr, 0)));
}

RsExpr *
ConverterRefCount::VisitImplicitCastExpr(clang::ImplicitCastExpr *expr) {
  auto *sub_expr = expr->getSubExpr();

  // return unique_ptr (implicit xvalue cast)
  if (expr->isXValue() && sub_expr->isLValue()) {
    auto *node = ConvertExpr(sub_expr);
    if (IsUniquePtr(sub_expr->getType())) {
      computed_expr_type_ = ComputedExprType::FreshValue;
      return Cat(node, Text(".take()"));
    }
    computed_expr_type_ = ComputedExprType::Value;
    return node;
  }

  if (auto *unary = clang::dyn_cast<clang::UnaryOperator>(sub_expr);
      expr->getCastKind() == clang::CastKind::CK_LValueToRValue && unary &&
      (unary->isPostfix() || unary->isPrefix())) {
    return ConvertExpr(sub_expr);
  }

  if (expr->getCastKind() == clang::CastKind::CK_BitCast) {
    if (expr->getType()->isVoidPointerType()) {
      if (sub_expr->getType()->isVoidPointerType()) {
        return ConvertExpr(sub_expr);
      }
      PushConversionKind push(*this, ConversionKind::Unboxed);
      RsExpr *node = nullptr;
      if (sub_expr->getType()->isPointerType() &&
          sub_expr->getType()->getPointeeType()->isArrayType()) {
        auto *ptr = ConvertFreshPointer(sub_expr);
        auto *elem_type = Convert(sub_expr->getType()
                                      ->getPointeeType()
                                      ->getAsArrayTypeUnsafe()
                                      ->getElementType());
        node = Cat(Text('('), ptr, Text("as Ptr<"), elem_type,
                   Text(">).to_any()"));
      } else if (IsStringLiteralExpr(sub_expr)) {
        auto *ptr = ConvertFreshPointer(sub_expr);
        node = Cat(ptr, Text(".to_any()"));
      } else {
        auto *ptr = ConvertFreshPointer(sub_expr);
        auto *type_node = Convert(sub_expr->getType());
        node = Cat(arena_.New<Cast>(ptr, type_node), Text(".to_any()"));
      }
      computed_expr_type_ = ComputedExprType::FreshPointer;
      return node;
    }
    if (sub_expr->getType()->isVoidPointerType() &&
        expr->getType()->isPointerType()) {
      auto *node = ConvertExpr(sub_expr);
      PushConversionKind push(*this, ConversionKind::Unboxed);
      node = Cat(node, Text(".reinterpret_cast::<"),
                 ConvertPointeeType(expr->getType()), Text(">()"));
      computed_expr_type_ = ComputedExprType::FreshPointer;
      return node;
    }
    return ConvertExpr(sub_expr);
  }

  if (expr->getCastKind() == clang::CastKind::CK_DerivedToBase) {
    if (expr->getType()->isPointerType()) {
      auto ptype = clang::dyn_cast<clang::PointerType>(expr->getType());
      auto pointee_type = ptype->getPointeeType()->getAsCXXRecordDecl();

      if (pointee_type && abstract_structs_.contains(GetID(pointee_type))) {
        PushConversionKind push(*this, ConversionKind::Unboxed);
        auto *node = ConvertExpr(sub_expr->IgnoreCasts());
        node = Cat(Text('('), node, Text(".to_strong() as Value<"),
                   ConvertPointeeType(expr->getType()), Text('>'),
                   Text(").as_pointer_dyn()"));
        computed_expr_type_ = ComputedExprType::FreshPointer;
        return node;
      }
    }
  }

  if (expr->getCastKind() == clang::CastKind::CK_ArrayToPointerDecay) {
    if (IsVaListType(sub_expr->getType())) {
      return ConvertExpr(sub_expr);
    }
    if (IsStringLiteralExpr(sub_expr)) {
      auto *node = ConvertExpr(sub_expr->IgnoreParens());
      return Cat(Text("Ptr::from_string_literal("), node, Text(')'));
    }
    // we need to write (var.as_pointer as Ptr<T>) because Rust isn't
    // smart enough to pick the right specialization
    PushConversionKind push(*this, ConversionKind::Unboxed);
    auto *ptr = ConvertPointer(sub_expr);
    auto *type_node = Convert(expr->getType());
    return arena_.New<Cast>(ptr, type_node);
  }

  if (expr->getCastKind() == clang::CastKind::CK_NullToPointer) {
    PushConversionKind push(*this, ConversionKind::Unboxed);
    computed_expr_type_ = ComputedExprType::FreshPointer;
    return GetDefaultAsString(expr->getType());
  }

  if (expr->getCastKind() == clang::CastKind::CK_NoOp) {
    return ConvertExpr(sub_expr);
  }

  return Converter::VisitImplicitCastExpr(expr);
}

RsExpr *ConverterRefCount::EmitFnPtrCall(clang::Expr *callee) {
  return Cat(Text("(*"), ConvertExpr(callee), Text(')'));
}

RsExpr *ConverterRefCount::ConvertFunctionPointerPlaceholder(
    clang::Expr *arg, [[maybe_unused]] std::string_view param_type) {
  return ConvertRValue(arg);
}

RsExpr *ConverterRefCount::ConvertFunctionToFunctionPointer(
    const clang::FunctionDecl *fn_decl) {
  computed_expr_type_ = ComputedExprType::FreshPointer;
  return Cat(Text("FnPtr::<"),
             ConvertFunctionPointerType(
                 fn_decl->getType()->getAs<clang::FunctionProtoType>()),
             Text(std::format(">::new({})", Mapper::MapFunctionName(fn_decl))));
}

RsExpr *ConverterRefCount::ConvertEqualsNullPtr(clang::Expr *expr) {
  auto *node = ConvertExpr(expr);
  computed_expr_type_ = ComputedExprType::FreshValue;
  return Cat(Text('('), node, Text(").is_null()"));
}

RsExpr *
ConverterRefCount::VisitFunctionPointerCast(clang::ExplicitCastExpr *expr) {
  if (expr->getType()->isFunctionPointerType() ||
      expr->getSubExpr()->getType()->isFunctionPointerType()) {
    if (expr->getSubExpr()->getType()->isFunctionPointerType() &&
        expr->getType()->isFunctionPointerType()) {
      auto target_proto =
          expr->getType()->getPointeeType()->getAs<clang::FunctionProtoType>();
      auto src_proto = expr->getSubExpr()
                           ->getType()
                           ->getPointeeType()
                           ->getAs<clang::FunctionProtoType>();
      auto *fn_type = ConvertFunctionPointerType(target_proto);

      RsExpr *adapter = Text("None");
      // Only accept direct references to the casted function. Otherwise the
      // closure would be capturing and would not coerce into a fn pointer.
      if (auto *decl_ref = clang::dyn_cast<clang::DeclRefExpr>(
              expr->getSubExpr()->IgnoreImplicit())) {
        if (auto *fn_decl =
                clang::dyn_cast<clang::FunctionDecl>(decl_ref->getDecl())) {
          adapter = BuildFnAdapter(fn_decl, src_proto, target_proto);
        }
      }

      auto *sub = ConvertExpr(expr->getSubExpr());
      return Cat(sub, Text(".cast::<"), fn_type, Text(">("), adapter,
                 Text(')'));
    }
    if (expr->getSubExpr()->getType()->isFunctionPointerType() ||
        expr->getType()->isVoidPointerType()) {
      auto *sub = ConvertExpr(expr->getSubExpr());
      return Cat(sub, Text(".to_any()"));
    }
    if (expr->getSubExpr()->getType()->isVoidPointerType() ||
        expr->getType()->isFunctionPointerType()) {
      auto target_proto =
          expr->getType()->getPointeeType()->getAs<clang::FunctionProtoType>();
      auto *fn_type = ConvertFunctionPointerType(target_proto);
      auto *sub = ConvertExpr(expr->getSubExpr());
      return Cat(sub, Text(".cast_fn::<"), fn_type,
                 Text(">().expect(\"ub:wrong fn type\")"));
    }
    assert(0 && "Unhandled function pointer cast");
    return Text("");
  }

  return nullptr;
}

RsExpr *
ConverterRefCount::VisitExplicitCastExpr(clang::ExplicitCastExpr *expr) {
  if (expr->getTypeAsWritten()->isVoidType()) {
    PushExprKind push(*this, ExprKind::Void);
    auto *node = ConvertExpr(expr->getSubExpr());
    if (!ExprIsCopyable(expr->getSubExpr()) && !isFresh()) {
      return Cat(node, Text(".clone()"));
    }
    return node;
  }
  if (expr->getCastKind() == clang::CK_NullToPointer) {
    PushConversionKind push(*this, ConversionKind::Unboxed);
    computed_expr_type_ = ComputedExprType::FreshPointer;
    return GetDefaultAsString(expr->getType());
  }
  switch (expr->getStmtClass()) {
  case clang::Stmt::CXXReinterpretCastExprClass: {
    assert(expr->getType()->isPointerType() &&
           "Only pointer casts are supported in reinterpret_cast");
    auto *sub = ConvertExpr(expr->getSubExpr());
    computed_expr_type_ = ComputedExprType::FreshPointer;
    return Cat(sub, Text(std::format(".reinterpret_cast::<{}>()",
                                     GetUnsafeTypeAsString(
                                         expr->getType()->getPointeeType()))));
  }
  case clang::Stmt::CStyleCastExprClass:
  case clang::Stmt::CXXStaticCastExprClass: {
    if (expr->getCastKind() == clang::CastKind::CK_PointerToIntegral ||
        expr->getCastKind() == clang::CastKind::CK_IntegralToPointer) {
      RsExpr *dst_type = nullptr;
      {
        PushConversionKind push(*this, ConversionKind::Unboxed);
        dst_type = Convert(expr->getType());
      }
      if (expr->getCastKind() == clang::CastKind::CK_PointerToIntegral) {
        auto *sub = ConvertExpr(expr->getSubExpr());
        computed_expr_type_ = ComputedExprType::FreshValue;
        return Cat(sub, Text(".to_int()"));
      }
      auto *sub = ConvertExpr(expr->getSubExpr());
      computed_expr_type_ = ComputedExprType::FreshPointer;
      if (GetSafeTypeAsString(expr->getSubExpr()->getType()) != "usize") {
        sub = Cat(Parens(sub), Text("as usize"));
      }
      return Cat(Text('<'), dst_type, Text(">::from_int("), sub, Text(')'));
    }

    if (auto *fn_cast = VisitFunctionPointerCast(expr)) {
      return fn_cast;
    }
    if (expr->getSubExpr()->getType()->isVoidPointerType() &&
        expr->getType()->isVoidPointerType()) {
      return ConvertExpr(expr->getSubExpr());
    }
    if (expr->getSubExpr()->getType()->isVoidPointerType() &&
        expr->getType()->isPointerType()) {
      auto *sub = ConvertExpr(expr->getSubExpr());
      PushConversionKind push(*this, ConversionKind::Unboxed);
      computed_expr_type_ = ComputedExprType::FreshPointer;
      return Cat(sub, Text(".reinterpret_cast::<"),
                 ConvertPointeeType(expr->getType()), Text(">()"));
    }
    if (expr->getType()->isVoidPointerType() &&
        expr->getSubExpr()->getType()->isPointerType()) {
      auto *sub = ConvertFreshPointer(expr->getSubExpr());
      computed_expr_type_ = ComputedExprType::FreshPointer;
      return Cat(sub, Text(".to_any()"));
    }
    if (expr->getSubExpr()->getType()->isPointerType() &&
        !expr->getSubExpr()->isNullPointerConstant(
            ctx_, clang::Expr::NPC_ValueDependentIsNull)) {
      auto *sub = ConvertExpr(expr->getSubExpr());
      computed_expr_type_ = ComputedExprType::FreshPointer;
      return Cat(sub, Text(".reinterpret_cast::<"),
                 ConvertPointeeType(expr->getType()), Text(">()"));
    }
    return Converter::VisitExplicitCastExpr(expr);
  }
  default:
    return ConvertExpr(expr->getSubExpr());
  }
}

bool ConverterRefCount::RustSizeofMatchesCSizeof(clang::QualType ty) const {
  ty = ty.getCanonicalType();
  if (ty->isArrayType() || ty->isPointerType() || ty->isReferenceType() ||
      ty->isMemberPointerType()) {
    return false;
  }
  if (auto *record = ty->getAsRecordDecl()) {
    auto *def = record->getDefinition();
    if (!def) {
      return false;
    }
    for (const auto *field : def->fields()) {
      if (field->isBitField() || !RustSizeofMatchesCSizeof(field->getType())) {
        return false;
      }
    }
    return true;
  }
  return true;
}

RsExpr *ConverterRefCount::VisitStmtExpr(clang::StmtExpr *expr) {
  PushConversionKind push(*this, ConversionKind::FullRefCount);
  return Converter::VisitStmtExpr(expr);
}

RsExpr *ConverterRefCount::EmitStmtExprTail(clang::Expr *tail) {
  auto *node = ConvertExpr(tail);
  SetFreshType(tail->getType());
  return Cat(Text("let __result = "), node, Text(token::kSemiColon),
             Text("__result"));
}

RsExpr *ConverterRefCount::ConvertBinaryOperator(clang::BinaryOperator *expr) {
  auto *lhs = expr->getLHS();
  auto *rhs = expr->getRHS();
  auto lhs_type = lhs->getType();
  auto rhs_type = rhs->getType();
  std::string_view opcode_as_string = expr->getOpcodeStr();

  if (auto *assign = llvm::dyn_cast<clang::CompoundAssignOperator>(expr);
      assign && GetSafeTypeAsString(lhs_type) !=
                    GetSafeTypeAsString(assign->getComputationResultType())) {
    auto computation_result_type = assign->getComputationResultType();
    RsExpr *value = nullptr;
    if (IsUnsignedArithOp(assign)) {
      auto *lhs_node = ConvertRValue(lhs);
      auto *receiver = Parens(CastTo(lhs_node, computation_result_type));
      auto *arith = ConvertUnsignedArithBinaryOperator(expr, rhs, receiver);
      value = Parens(arith);
    } else {
      auto *lhs_node = ConvertRValue(lhs);
      auto op = opcode_as_string;
      op.remove_suffix(1); // remove '=' from operator
      auto *rhs_node = ConvertRValue(rhs);
      value = Parens(Cat(Parens(CastTo(lhs_node, computation_result_type)),
                         Text(std::string(op)), rhs_node));
    }
    if (lhs_type->isBooleanType()) {
      value = Cat(value, Text(token::kDiff), Text(token::kZero));
    } else {
      value = CastTo(value, lhs_type);
    }
    auto *assign_node = arena_.New<Assign>(ConvertLValue(lhs), Text("rhs_0"));
    auto *node = Cat(Text(keyword::kLet), Text("rhs_0"), Text(token::kAssign),
                     value, Text(token::kSemiColon), assign_node);
    if (isRValue()) {
      node = Cat(node, Text(token::kSemiColon), ConvertFreshRValue(lhs));
    }
    return Braces(node);
  }

  if (IsUnsignedArithOp(expr)) {
    RsExpr *operand = nullptr;
    if (expr->isCompoundAssignmentOp() && lhs->isLValue()) {
      operand = Parens(ConvertRValue(lhs));
    } else {
      operand = Parens(ConvertUnsignedArithOperand(lhs, expr->getType()));
    }
    auto *arith = ConvertUnsignedArithBinaryOperator(expr, rhs, operand);
    if (expr->isCompoundAssignmentOp()) {
      auto *assign_node = arena_.New<Assign>(ConvertLValue(lhs), Text("rhs_0"));
      auto *node = Cat(Text(keyword::kLet), Text("rhs_0"), Text(token::kAssign),
                       arith, Text(token::kSemiColon), assign_node);
      if (isRValue()) {
        node = Cat(node, Text(token::kSemiColon), ConvertFreshRValue(lhs));
      }
      return Braces(node);
    }
    computed_expr_type_ = ComputedExprType::FreshValue;
    return arith;
  }

  // pointer subtraction. The Sub trait gets elements by Value, so we need
  // fresh pointers
  if (expr->isAdditiveOp() && lhs_type->isPointerType() &&
      rhs_type->isPointerType()) {
    auto *lhs_node = ConvertFreshPointer(lhs);
    auto *rhs_node = ConvertFreshPointer(rhs);
    computed_expr_type_ = ComputedExprType::FreshValue;
    return CastTo(Parens(Cat(lhs_node, Text(std::string(expr->getOpcodeStr())),
                             rhs_node)),
                  expr->getType());
  }

  if (expr->isAssignmentOp()) {
    return ConvertAssignment(lhs, rhs, opcode_as_string);
  }

  return Converter::ConvertBinaryOperator(expr);
}

static bool IsZeroInitExpr(clang::ASTContext &ctx, const clang::Expr *expr) {
  if (clang::isa<clang::ImplicitValueInitExpr>(expr)) {
    return true;
  }
  if (auto list = clang::dyn_cast<clang::InitListExpr>(expr)) {
    return std::all_of(
        list->inits().begin(), list->inits().end(),
        [&](const clang::Expr *init) { return IsZeroInitExpr(ctx, init); });
  }
  if (expr->isNullPointerConstant(ctx, clang::Expr::NPC_ValueDependentIsNull) !=
      clang::Expr::NPCK_NotNull) {
    return true;
  }
  clang::Expr::EvalResult result;
  return expr->EvaluateAsRValue(result, ctx) && result.Val.isInt() &&
         result.Val.getInt() == 0;
}

static std::optional<std::vector<uint8_t>>
GetConstantUnionBytes(clang::ASTContext &ctx, const clang::InitListExpr *expr) {
  if (expr->getNumInits() != 1) {
    return std::nullopt;
  }
  const auto *field = expr->getInitializedFieldInUnion();
  if (!field) {
    return std::nullopt;
  }
  const auto *init = expr->getInit(0)->IgnoreParenImpCasts();
  uint64_t union_size = ctx.getTypeSize(expr->getType()) / 8;
  std::vector<uint8_t> bytes;
  if (const auto *str = clang::dyn_cast<clang::StringLiteral>(init);
      str && str->getCharByteWidth() == 1) {
    auto data = str->getString();
    bytes.assign(data.begin(), data.end());
  } else {
    clang::Expr::EvalResult result;
    if (!init->EvaluateAsRValue(result, ctx)) {
      return std::nullopt;
    }
    uint64_t field_size = ctx.getTypeSize(field->getType()) / 8;
    llvm::APInt value;
    if (result.Val.isInt()) {
      value = result.Val.getInt();
    } else if (result.Val.isFloat()) {
      value = result.Val.getFloat().bitcastToAPInt();
    } else {
      return std::nullopt;
    }
    value = value.zextOrTrunc(field_size * 8);
    for (uint64_t i = 0; i < field_size; ++i) {
      bytes.push_back(value.extractBitsAsZExtValue(8, i * 8));
    }
  }
  if (bytes.size() > union_size) {
    return std::nullopt;
  }
  bytes.resize(union_size, 0);
  return bytes;
}

RsExpr *ConverterRefCount::VisitInitListExpr(clang::InitListExpr *expr) {
  if (auto form = expr->getSemanticForm())
    expr = form;

  auto qual_type = expr->getType();
  if (qual_type->isScalarType()) {
    PushConversionKind push(*this, ConversionKind::Unboxed);
    auto *node = Converter::VisitInitListExpr(expr);
    computed_expr_type_ = ComputedExprType::FreshValue;
    return node;
  }

  if (qual_type->isRecordType()) {
    if (IsZeroInitExpr(ctx_, expr)) {
      if (auto init = Mapper::MapInitializer(qual_type); !init.empty()) {
        computed_expr_type_ = ComputedExprType::FreshValue;
        return Text(std::move(init));
      }
    }
    const auto *record = qual_type->getAsRecordDecl();
    if (record->getQualifiedNameAsString() == "std::array") {
      RsExpr *node = nullptr;
      if (auto init = clang::dyn_cast<clang::InitListExpr>(expr->getInit(0))) {
        PushConversionKind push(*this, ConversionKind::Unboxed);
        node = ConverterRefCount::VisitInitListExpr(init);
      } else {
        node = Text("[]");
      }
      computed_expr_type_ = ComputedExprType::FreshValue;
      return Cat(Text("vec!"), node);
    }

    if (record->isUnion()) {
      computed_expr_type_ = ComputedExprType::FreshValue;
      if (expr->getNumInits() == 0 || IsZeroInitExpr(ctx_, expr->getInit(0))) {
        return Text("Default::default()");
      }
      auto bytes = GetConstantUnionBytes(ctx_, expr);
      assert(bytes && "unsupported non-zero union initializer");
      std::string list;
      for (auto byte : *bytes) {
        list += std::to_string(byte);
        list += ',';
      }
      return Text(std::format(
          "{} {{ __bytes: Rc::new(RefCell::new(Box::from([{}]))) }}",
          GetRecordName(record), list));
    }

    std::vector<RsExpr *> fields;
    {
      int i = 0;
      PushConversionKind push(*this, ConversionKind::UnboxedField);
      for (const auto *field : record->fields()) {
        fields.push_back(Text(GetNamedDeclAsString(field)));
        fields.push_back(Text(token::kColon));
        fields.push_back(ConvertVarInit(field->getType(), expr->getInit(i++)));
        fields.push_back(Text(token::kComma));
      }
    }
    computed_expr_type_ = ComputedExprType::FreshValue;
    return Cat(Text(GetUnsafeTypeAsString(qual_type)),
               Braces(arena_.New<Concat>(std::move(fields))));
  }

  if (IsInitExprOfStringLiteral(expr)) {
    auto *node = ConvertExpr(expr->getInit(0)->IgnoreParenImpCasts());
    computed_expr_type_ = ComputedExprType::FreshValue;
    return node;
  }

  auto conv = getConversionKind();
  bool nested_array =
      expr->getNumInits() > 0 && expr->getInit(0)->getType()->isArrayType();
  // 2D arrays are FullRefCount'ed on the second level as well, including the
  // ones held by an unboxed field.
  PushConversionKind push(
      *this,
      nested_array ? ConversionKind::FullRefCount : ConversionKind::Unboxed,
      !nested_array || conv == ConversionKind::UnboxedField);

  RsExpr *node = nullptr;
  switch (conv) {
  case ConversionKind::Unboxed:
  case ConversionKind::Ptr:
    node = Converter::VisitInitListExpr(expr);
    break;
  case ConversionKind::UnboxedField:
  case ConversionKind::FullRefCount:
    node =
        Cat(Text("Box::new("), Converter::VisitInitListExpr(expr), Text(')'));
    break;
  }
  computed_expr_type_ = ComputedExprType::FreshValue;
  return node;
}

RsExpr *ConverterRefCount::ConvertUnionMemberAccessor(clang::MemberExpr *expr) {
  auto member_type = expr->getMemberDecl()->getType();
  bool is_array = member_type->isArrayType();
  auto *node = ConvertMemberBytePtr(
      expr, is_array ? member_type->getAsArrayTypeUnsafe()->getElementType()
                     : member_type);

  if (isAddrOf() || is_array) {
    return node;
  }
  node = arena_.New<Unary>(Unary::Op::Deref, node);
  if (!isLValue()) {
    SetValueFreshness(member_type);
  }
  return node;
}

RsExpr *ConverterRefCount::ConvertMemberBytePtr(clang::MemberExpr *expr,
                                                clang::QualType elem_type) {
  uint64_t byte_off = 0;
  clang::Expr *base = expr;
  while (auto *member = clang::dyn_cast<clang::MemberExpr>(base)) {
    const auto *field = clang::cast<clang::FieldDecl>(member->getMemberDecl());
    const auto &layout = ctx_.getASTRecordLayout(field->getParent());
    byte_off += layout.getFieldOffset(field->getFieldIndex()) / 8;
    base = member->getBase();
    if (member->isArrow()) {
      break;
    }
    base = base->IgnoreParenImpCasts();
  }

  RsExpr *node = ConvertPointer(base);
  node = MethodCall(node, "reinterpret_cast::<u8>", std::vector<RsExpr *>{},
                    /*is_mut=*/false);
  node =
      MethodCall(node, "offset",
                 std::vector<RsExpr *>{Text(std::format("{}usize", byte_off))},
                 /*is_mut=*/false);
  PushConversionKind push(*this, ConversionKind::Unboxed);
  auto elem_name = RenderType(elem_type);
  if (elem_name != "u8") {
    node = MethodCall(node, std::format("reinterpret_cast::<{}>", elem_name),
                      std::vector<RsExpr *>{}, /*is_mut=*/false);
  }
  computed_expr_type_ = ComputedExprType::FreshPointer;
  return arena_.New<Cast>(node, Text(std::format("Ptr<{}>", elem_name)));
}

RsExpr *ConverterRefCount::TryFlexibleArrayMember(clang::MemberExpr *expr) {
  if (!IsFlexibleArrayMemberAccess(ctx_, expr)) {
    return nullptr;
  }
  return ConvertMemberBytePtr(
      expr, expr->getType()->getAsArrayTypeUnsafe()->getElementType());
}

RsExpr *ConverterRefCount::ConvertFieldPtr(clang::MemberExpr *expr,
                                           const clang::FieldDecl *field) {
  if (auto *fam = TryFlexibleArrayMember(expr)) {
    return fam;
  }

  auto *base = expr->getBase();
  auto base_type = expr->isArrow() ? base->getType()->getPointeeType()
                                   : base->getType().getNonReferenceType();

  auto *parent = ConvertPointer(base);

  PushConversionKind push(*this, ConversionKind::Unboxed);
  auto base_type_name = RenderType(base_type);

  const auto &layout = ctx_.getASTRecordLayout(field->getParent());
  auto byte_off = layout.getFieldOffset(field->getFieldIndex()) / 8;
  bool container =
      field->getType()->isArrayType() || IsBoxedType(field->getType());

  computed_expr_type_ = ComputedExprType::FreshPointer;
  return arena_.New<FieldPtr>(parent, byte_off, std::move(base_type_name),
                              GetNamedDeclAsString(field), container);
}

RsExpr *ConverterRefCount::VisitMemberExpr(clang::MemberExpr *expr) {
  auto *member = expr->getMemberDecl();
  bool known = Mapper::Contains(expr);

  if (auto *method = clang::dyn_cast<clang::CXXMethodDecl>(member);
      method && !known) {
    // User-defined types have Value<T> fields; the struct itself is read-only
    // and only needs an immutable borrow. Non-user-defined types (STL)
    // need a mutable borrow for non-const methods
    auto base_type = expr->getBase()->getType().getNonReferenceType();
    if (base_type->isPointerType()) {
      base_type = base_type->getPointeeType();
    }
    if (IsMethodOnPtr(method)) {
      auto *receiver = ConvertPointer(expr->getBase());
      auto name = IsOverloadedMethod(method) ? GetOverloadedFunctionName(method)
                                             : GetNamedDeclAsString(method);
      return arena_.New<Field>(receiver, std::move(name));
    }
    bool needs_mut = NeedsMutAccess(method, base_type);
    PushExprKind push(*this, needs_mut ? ExprKind::LValue : ExprKind::RValue);
    return Converter::ConvertMemberExpr(expr);
  }

  if (auto *parent =
          clang::dyn_cast<clang::RecordDecl>(member->getDeclContext());
      parent && parent->isUnion() && clang::isa<clang::FieldDecl>(member)) {
    return ConvertUnionMemberAccessor(expr);
  }

  if (auto *field = clang::dyn_cast<clang::FieldDecl>(member);
      field && isAddrOf() && !known && !field->getType()->isReferenceType()) {
    return ConvertFieldPtr(expr, field);
  }

  RsExpr *node = nullptr;
  if (known) {
    node = GetMappedAsNode(expr);
    if (!node) {
      node = Text("");
    }
  } else if (isLValue()) {
    node = Converter::ConvertMemberExpr(expr);
  } else {
    PushExprKind push(*this, ExprKind::RValue);
    node = Converter::ConvertMemberExpr(expr);
  }

  if (isAddrOf()) {
    if (member->getType()->isReferenceType()) {
      computed_expr_type_ = ComputedExprType::Pointer;
      return node;
    }
    computed_expr_type_ = ComputedExprType::FreshPointer;
    return Cat(node, Text(".as_pointer()"));
  }

  if (member->getType()->isReferenceType()) {
    if (isLValue()) {
      return arena_.New<Unary>(Unary::Op::Deref, node);
    }
    node = arena_.New<Unary>(Unary::Op::Deref, node);
  } else if (isRValue()) {
    if (known) {
      node = arena_.New<BorrowRead>(node);
    }
  } else if (known) {
    node = arena_.New<BorrowWrite>(node);
  }
  SetValueFreshness(expr->getType());
  return node;
}

RsExpr *ConverterRefCount::VisitCXXNewExpr(clang::CXXNewExpr *expr) {
  RsExpr *node = nullptr;
  if (expr->isArray()) {
    if (auto *init = llvm::dyn_cast_or_null<clang::InitListExpr>(
            expr->getInitializer())) {
      node = Cat(Text("Ptr::alloc_array("), ConvertExpr(init), Text(')'));
    } else {
      auto *array_size = ConvertExpr(*expr->getArraySize());
      auto alloc_type = expr->getAllocatedType();
      PushConversionKind push(*this, ConversionKind::Unboxed);
      auto *alloc_type_node = Convert(alloc_type);
      auto *default_alloc_node = GetDefaultAsString(alloc_type);

      node = Cat(Text("Ptr::alloc_array((0.."), array_size, Text(").map(|_|"),
                 default_alloc_node, Text(").collect::<Box<["), alloc_type_node,
                 Text("]>>())"));
    }
  } else {
    RsExpr *init = nullptr;
    if (expr->getInitializer() == nullptr) {
      init = Text("Default::default()");
    } else {
      init = ConvertExpr(expr->getInitializer());
    }
    node = Cat(Text("Ptr::alloc("), init, Text(')'));
  }
  computed_expr_type_ = ComputedExprType::FreshPointer;
  return node;
}

RsExpr *ConverterRefCount::VisitCXXDeleteExpr(clang::CXXDeleteExpr *expr) {
  auto *argument = ConvertExpr(expr->getArgument());
  if (expr->isArrayForm()) {
    return Cat(argument, Text(".delete_array()"));
  }
  return Cat(argument, Text(".delete()"));
}

RsExpr *ConverterRefCount::EmitByValueShadow(const std::string &loop_var_name,
                                             clang::QualType type,
                                             RsExpr *box_expr,
                                             const std::string &type_override) {
  if (!type->isReferenceType()) {
    PushConversionKind push(*this, ConversionKind::FullRefCount);
    auto *type_node =
        type_override.empty() ? Convert(type) : Text(type_override);
    return Cat(Text(keyword::kLet), Text(loop_var_name), Text(token::kColon),
               type_node, Text(token::kAssign), BoxValue(box_expr),
               Text(token::kSemiColon));
  }
  return Text("");
}

RsExpr *
ConverterRefCount::VisitCXXForRangeStmtMap(clang::CXXForRangeStmt *stmt) {
  auto *loop_var = stmt->getLoopVariable();
  auto loop_var_name = GetNamedDeclAsString(loop_var);

  auto *range_init = ConvertObject(stmt->getRangeInit());

  auto *shadow = EmitByValueShadow(
      loop_var_name, loop_var->getType(), Text(loop_var_name),
      "Value<" + Mapper::Map(GetForRangeIteratorType(stmt)) + '>');

  auto *body = ConvertForRangeBody(stmt, loop_var);

  return Cat(Text("'loop_:"), Text(keyword::kFor), Text(loop_var_name),
             Text(keyword::kIn), Text("RefcountMapIter::begin("), range_init,
             Text(')'), Braces(Cat(shadow, body)));
}

RsExpr *
ConverterRefCount::VisitCXXForRangeStmtVector(clang::CXXForRangeStmt *stmt) {
  auto *loop_var = stmt->getLoopVariable();
  auto loop_var_name = GetNamedDeclAsString(loop_var);

  auto *range_init = ConvertObject(stmt->getRangeInit());
  auto *ptr_type = ConvertPtrType(stmt->getRangeInit()->getType());

  RsExpr *shadow = nullptr;
  // handle multi-level types such as Vec<Value<Vec<T>>>
  if (IsBoxedType(stmt->getRangeInit()->getType()) &&
      GetInnerType(stmt->getRangeInit()->getType()).starts_with("Value<")) {
    std::vector<RsExpr *> parts;
    parts.push_back(Text(keyword::kLet));
    parts.push_back(Text(loop_var_name));
    parts.push_back(Text(token::kColon));

    if (loop_var->getType()->isReferenceType()) {
      parts.push_back(Convert(loop_var->getType()));
      parts.push_back(Text(token::kAssign));
      parts.push_back(arena_.New<Unary>(Unary::Op::Deref, Text(loop_var_name)));
      parts.push_back(Text(".as_pointer()"));
    } else {
      PushConversionKind push(*this, ConversionKind::FullRefCount);
      parts.push_back(Convert(loop_var->getType()));
      parts.push_back(Text(token::kAssign));
      parts.push_back(Text("Rc::new(RefCell::new("));
      parts.push_back(arena_.New<Unary>(Unary::Op::Deref, Text(loop_var_name)));
      parts.push_back(Text(".borrow().clone()))"));
    }
    parts.push_back(Text(token::kSemiColon));
    shadow = arena_.New<Concat>(std::move(parts));
  } else {
    shadow = EmitByValueShadow(
        loop_var_name, loop_var->getType(),
        Cat(arena_.New<Unary>(Unary::Op::Deref, Text(loop_var_name)),
            Text(".clone()")));
  }

  auto *body = ConvertForRangeBody(stmt);

  return Cat(
      Text("'loop_:"), Text(keyword::kFor),
      Text(stmt->getLoopVariable()->getType().isConstQualified() ? "" : "mut"),
      Text(loop_var_name), Text(keyword::kIn),
      arena_.New<Cast>(range_init, ptr_type), Braces(Cat(shadow, body)));
}

RsExpr *
ConverterRefCount::VisitCXXForRangeStmtString(clang::CXXForRangeStmt *stmt) {
  auto *loop_var = stmt->getLoopVariable();
  auto loop_var_name = GetNamedDeclAsString(loop_var);

  auto *range_init = ConvertObject(stmt->getRangeInit());
  auto *iter_type = Convert(loop_var->getType().getNonReferenceType());

  auto *shadow = EmitByValueShadow(
      loop_var_name, loop_var->getType(),
      Cat(arena_.New<Unary>(Unary::Op::Deref, Text(loop_var_name)),
          Text(".clone()")));
  auto *body = ConvertForRangeBody(stmt);

  return Cat(
      Text("'loop_:"), Text(keyword::kFor),
      Text(stmt->getLoopVariable()->getType().isConstQualified() ? "" : "mut"),
      Text(loop_var_name), Text(keyword::kIn), range_init,
      Text(".to_string_iterator() as StringIterator<"), iter_type, Text('>'),
      Braces(Cat(shadow, body)));
}

RsExpr *
ConverterRefCount::ConvertArrayCXXConstructExpr(clang::CXXConstructExpr *expr) {
  auto *args = ConvertCXXConstructExprArgs(expr);
  return Cat(Text("Box::new"),
             Parens(Cat(Text(std::format("std::array::from_fn::<_, {}, _>",
                                         GetArraySize(expr->getType()))),
                        Parens(Cat(Text("|_|"), args)))));
}

RsExpr *ConverterRefCount::ConvertStream(clang::Expr *expr) {
  return ConvertPointer(expr);
}

RsExpr *
ConverterRefCount::VisitCXXConstructExpr(clang::CXXConstructExpr *expr) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  PushSuppressIteratorClone push_suppress(*this, expr);

  if (auto *mapped =
          GetMappedAsNode(expr, expr->getArgs(), expr->getNumArgs())) {
    if (isAddrOf()) {
      computed_expr_type_ = ComputedExprType::FreshPointer;
      return Cat(Text("Rc::new(RefCell::new("), mapped,
                 Text(")).as_pointer()"));
    }
    computed_expr_type_ = ComputedExprType::FreshValue;
    return mapped;
  }

  auto *ctor = expr->getConstructor();
  if (ctor->isMoveConstructor() ||
      (ctor->isConvertingConstructor(false) && ctor->getNumParams() == 1 &&
       ctor->getParamDecl(0)->getType()->isRValueReferenceType())) {
    return ConvertLValue(expr->getArg(0));
  }

  if (ctor->isCopyConstructor()) {
    return PushSuppressIteratorClone::take(*this)
               ? ConvertRValue(expr->getArg(0))
               : ConvertFreshRValue(expr->getArg(0));
  }

  if (ctor->isDefaultConstructor() && !ctor->isUserProvided()) {
    auto ty = expr->getType();
    auto *node = GetDefaultAsString(ty);
    SetFreshType(ty);
    return node;
  }

  assert(ctor->isUserProvided());
  RsExpr *node = nullptr;
  if (expr->getType()->isArrayType()) {
    node = ConvertArrayCXXConstructExpr(expr);
  } else {
    node = ConvertCXXConstructExprArgs(expr);
  }
  SetFreshType(expr->getType());
  return node;
}

RsExpr *ConverterRefCount::VisitImplicitValueInitExpr(
    clang::ImplicitValueInitExpr *expr) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  if (auto arr_ty = clang::dyn_cast<clang::ArrayType>(
          expr->getType()->getCanonicalTypeInternal().getTypePtr())) {
    if (clang::isa<clang::ConstantArrayType>(arr_ty)) {
      auto *node = Converter::VisitImplicitValueInitExpr(expr);
      computed_expr_type_ = ComputedExprType::FreshValue;
      return Cat(Text("Box::new("), node, Text(')'));
    }
  }

  return Converter::VisitImplicitValueInitExpr(expr);
}

RsExpr *ConverterRefCount::ConvertVariadicArg(clang::Expr *arg) {
  if (arg->getType()->isPointerType()) {
    return ConvertFreshPointer(arg);
  }
  return ConvertExpr(arg);
}

RsExpr *ConverterRefCount::VisitVAArgExpr(clang::VAArgExpr *expr) {
  auto va_list_expr = expr->getSubExpr();
  if (auto *cast = clang::dyn_cast<clang::ImplicitCastExpr>(va_list_expr)) {
    va_list_expr = cast->getSubExpr();
  }
  auto *va_list = ConvertLValue(va_list_expr);
  RsExpr *type_node = nullptr;
  {
    PushConversionKind push(*this, ConversionKind::Unboxed);
    type_node = Convert(expr->getType());
  }
  SetFreshType(expr->getType());
  return Cat(va_list, Text(".arg::<"), type_node, Text(">()"));
}

RsExpr *
ConverterRefCount::VisitCXXDefaultArgExpr(clang::CXXDefaultArgExpr *expr) {
  return Converter::VisitCXXDefaultArgExpr(expr);
}

RsExpr *ConverterRefCount::GetArrayDefaultAsString(clang::QualType qual_type) {
  if (auto *array_type =
          clang::dyn_cast<clang::IncompleteArrayType>(qual_type)) {
    PushConversionKind push(*this, ConversionKind::Unboxed);
    return Cat(Text("<Box<["), Convert(array_type->getElementType()),
               Text("]>>::default()"));
  }
  if (auto *array_type = clang::dyn_cast<clang::ConstantArrayType>(qual_type)) {
    const auto &size = array_type->getSize();
    auto size_as_string = GetNumAsString(size);
    auto element_type = array_type->getElementType();
    PushConversionKind push(*this, element_type->isArrayType()
                                       ? ConversionKind::FullRefCount
                                       : ConversionKind::Unboxed);
    auto *element_type_node = Convert(element_type);
    auto *default_node = GetDefaultAsString(element_type);
    return Cat(Text(std::format("(0..{}).map(|_|", size_as_string.c_str())),
               default_node, Text(").collect::<Box<["), element_type_node,
               Text("]>>()"));
  }
  return Converter::GetArrayDefaultAsString(qual_type);
}

RsExpr *ConverterRefCount::GetDefaultAsString(clang::QualType qual_type) {
  if (IsVaListType(qual_type)) {
    computed_expr_type_ = ComputedExprType::FreshValue;
    return BoxValue(Text("VaList::default()"));
  }

  if (auto *arr = GetArrayDefaultAsString(qual_type)) {
    computed_expr_type_ = ComputedExprType::FreshValue;
    return BoxValue(arr);
  }

  if (auto init = Mapper::MapInitializer(qual_type); !init.empty()) {
    computed_expr_type_ = ComputedExprType::FreshValue;
    return BoxValue(Text(std::move(init)));
  }

  RsExpr *ret = nullptr;
  if (qual_type->isPointerType()) {
    auto pointee_type = qual_type->getPointeeType();
    if (pointee_type->isFunctionType()) {
      auto *proto = pointee_type->getAs<clang::FunctionProtoType>();
      assert(proto && "Function pointer default without a prototype");
      ret = Cat(Text("FnPtr::<"), ConvertFunctionPointerType(proto),
                Text(">::null()"));
    } else {
      if (pointee_type->isVoidType()) {
        ret = Text("AnyPtr::default()");
      } else {
        PushConversionKind push(*this, ConversionKind::Unboxed);
        ret = Cat(Text("Ptr::<"), ConvertPointeeType(qual_type),
                  Text(">::null()"));
      }
    }
  } else {
    return Converter::GetDefaultAsString(qual_type);
  }
  computed_expr_type_ = ComputedExprType::FreshPointer;
  return BoxValue(ret);
}

RsExpr *
ConverterRefCount::GetDefaultAsStringFallback(clang::QualType qual_type) {
  return Cat(Text('<'), Convert(qual_type), Text(">::default()"));
}

RsExpr *ConverterRefCount::ConvertVarDefaultInit(clang::QualType qual_type) {
  PushConversionKind push(*this, ConversionKind::FullRefCount);
  return GetDefaultAsString(qual_type);
}

std::vector<const char *>
ConverterRefCount::GetStructAttributes(const clang::RecordDecl *decl) {
  std::vector<const char *> attrs;

  if (decl->isUnion()) {
    return attrs;
  }

  if (!clang::isa<clang::CXXRecordDecl>(decl) &&
      TypeImplementsClone(ctx_.getCanonicalTagType(decl))) {
    attrs.emplace_back("Clone");
  }

  if (RecordDerivesDefault(decl)) {
    attrs.emplace_back("Default");
  }
  return attrs;
}

RsExpr *ConverterRefCount::ConvertVarInitValue(clang::QualType qual_type,
                                               clang::Expr *expr) {
  if (auto lambda = clang::dyn_cast<clang::LambdaExpr>(
          expr->IgnoreUnlessSpelledInSource())) {
    PushConversionKind push(*this, ConversionKind::Unboxed);
    if (qual_type->isFunctionPointerType() && lambda->capture_size() == 0) {
      auto *node = VisitLambdaExpr(lambda);
      return Cat(Text("FnPtr::new("), node, Text(')'));
    }
    return VisitLambdaExpr(lambda);
  }

  PushInitType init_type(*this, qual_type);
  if (qual_type->isReferenceType() || qual_type->isFunctionPointerType()) {
    return ConvertFreshPointer(expr);
  }
  return ConvertFreshRValue(expr, qual_type);
}

RsExpr *ConverterRefCount::ConvertVarInit(clang::QualType qual_type,
                                          clang::Expr *expr) {
  bool is_ref = qual_type->isReferenceType();
  PushConversionKind push(*this, ConversionKind::Unboxed, is_ref);
  return BoxValue(ConvertVarInitValue(qual_type, expr));
}

RsExpr *ConverterRefCount::ConvertAssignment(clang::Expr *lhs, clang::Expr *rhs,
                                             std::string_view assign_operator) {
  auto *rhs_node = ConvertFreshRValue(rhs, lhs->getType());

  std::vector<RsExpr *> parts;
  bool hoisted_rhs = MayCauseBorrowMutError(lhs, rhs);
  if (hoisted_rhs) {
    parts.push_back(Cat(Text(keyword::kLet), Text("__rhs"),
                        Text(token::kAssign), rhs_node,
                        Text(token::kSemiColon)));
    rhs_node = Text("__rhs");
  }

  auto *lhs_node = ConvertLValue(lhs);
  if (assign_operator == "=") {
    parts.push_back(arena_.New<Assign>(lhs_node, rhs_node));
  } else {
    parts.push_back(arena_.New<CompoundAssign>(
        lhs_node, std::string(assign_operator), rhs_node));
  }

  if (!isVoid()) {
    parts.push_back(Text(token::kSemiColon));
    parts.push_back(ConvertFreshRValue(lhs));
  }
  return Braces(arena_.New<Concat>(std::move(parts)), !isVoid() || hoisted_rhs);
}

RsExpr *
ConverterRefCount::ConvertGenericBinaryOperator(clang::BinaryOperator *expr) {
  auto lhs = expr->getLHS();
  auto rhs = expr->getRHS();
  std::string_view opcode = expr->getOpcodeStr();

  auto lhs_vars = GetAllVars(lhs);
  auto rhs_vars = GetAllVars(rhs);

  auto predicate = [](auto *var) {
    return var->getType()->isPointerType() || var->getType()->isReferenceType();
  };

  auto sides_contains_literal = rhs_vars.empty() || lhs_vars.empty();
  auto same_var_on_both_sides = lhs_vars == rhs_vars;
  auto sides_contain_ptr_or_deref = std::ranges::any_of(rhs_vars, predicate) ||
                                    std::ranges::any_of(lhs_vars, predicate);

  auto both_sides_have_va_arg = same_var_on_both_sides &&
                                ContainsVAArgExpr(lhs) &&
                                ContainsVAArgExpr(rhs);

  auto may_cause_borrow_mut_err =
      both_sides_have_va_arg ||
      (!sides_contains_literal && !same_var_on_both_sides &&
       sides_contain_ptr_or_deref);

  if (may_cause_borrow_mut_err) {
    auto *lhs_node = ConvertFreshRValue(
        lhs, GetOperandImplicitConversionTarget(expr, lhs, rhs));
    auto *rhs_node = ConvertFreshRValue(
        rhs, GetOperandImplicitConversionTarget(expr, rhs, lhs));
    computed_expr_type_ = ComputedExprType::FreshValue;
    return Braces(Cat(Text("let _lhs ="), lhs_node, Text(token::kSemiColon),
                      Text("_lhs"), Text(std::string(opcode)), rhs_node));
  }

  auto *lhs_node =
      ConvertExpr(lhs, GetOperandImplicitConversionTarget(expr, lhs, rhs));
  auto *rhs_node =
      ConvertExpr(rhs, GetOperandImplicitConversionTarget(expr, rhs, lhs));
  computed_expr_type_ = ComputedExprType::FreshValue;
  return Parens(Cat(lhs_node, Text(std::string(opcode)), rhs_node));
}

RsExpr *
ConverterRefCount::ConvertUniquePtrDeref(clang::CXXOperatorCallExpr *expr) {
  if (isAddrOf()) {
    auto *node = ConvertRValue(expr->getArg(0));
    computed_expr_type_ = ComputedExprType::FreshPointer;
    return Cat(node, Text(".as_pointer()"));
  }
  auto *node = ConvertExpr(expr->getArg(0));
  SetValueFreshness(expr->getType());
  return Cat(Text("(*"), node,
             Text(std::format(".as_ref().unwrap().borrow{}())",
                              isRValue() ? "" : "_mut")));
}

RsExpr *ConverterRefCount::ConvertCXXOperatorCallExpr(
    clang::CXXOperatorCallExpr *expr) {
  switch (expr->getOperator()) {
  case clang::OverloadedOperatorKind::OO_Equal:
    return ConvertAssignment(expr->getArg(0), expr->getArg(1), "=");

  case clang::OverloadedOperatorKind::OO_Arrow:
  case clang::OverloadedOperatorKind::OO_Star: {
    if (IsUniquePtr(expr->getArg(0)->getType())) {
      return ConvertUniquePtrDeref(expr);
    }

    if (isLValue()) {
      return arena_.New<Unary>(Unary::Op::Deref, ConvertExpr(expr->getArg(0)));
    }

    if (GetStrongestIteratorCategory(expr->getArg(0)->getType()) ==
        IteratorCategory::Bidirectional) {
      return ConvertExpr(expr->getArg(0));
    }

    if (isAddrOf()) {
      return ConvertExpr(expr->getArg(0));
    }
    auto *arg = ConvertExpr(expr->getArg(0));
    auto *node = arena_.New<Unary>(Unary::Op::Deref, arg);
    SetValueFreshness(expr->getType());
    return node;
  }

  case clang::OverloadedOperatorKind::OO_Subscript: {
    if (IsUniquePtr(expr->getArg(0)->getType())) {
      auto *base = ConvertRValue(expr->getArg(0));
      auto *node = Cat(base, Text(".as_ref().unwrap()"));
      if (isAddrOf()) {
        auto *idx = ConvertRValue(expr->getArg(1));
        PushConversionKind push(*this, ConversionKind::Unboxed);
        auto *ptr = arena_.New<Cast>(
            MethodCall(node, "as_pointer", std::vector<RsExpr *>{},
                       /*is_mut=*/false),
            Cat(Text("Ptr<"), Convert(expr->getType()), Text('>')));
        node = MethodCall(ptr, "offset", std::vector<RsExpr *>{Parens(idx)},
                          /*is_mut=*/false);
      } else {
        auto *idx = ConvertRValue(expr->getArg(1));
        node = arena_.New<Index>(
            isRValue() ? static_cast<RsExpr *>(arena_.New<BorrowRead>(node))
                       : arena_.New<BorrowWrite>(node),
            idx);
      }
      SetValueFreshness(expr->getType());
      return node;
    }

    bool is_inner_boxed =
        IsBoxedType(expr->getType().getNonReferenceType()) &&
        IsBoxedType(expr->getArg(0)->getType().getNonReferenceType());

    if (isLValue()) {
      PushConversionKind push_ck(*this, ConversionKind::Unboxed);
      auto *object = ConvertObject(expr->getArg(0));
      auto *ptr_type = ConvertPtrType(expr->getArg(0)->getType());
      auto *idx = ConvertSubscriptIndex(expr->getArg(1));
      return arena_.New<Unary>(Unary::Op::Deref,
                               MethodCall(arena_.New<Cast>(object, ptr_type),
                                          "offset", std::vector<RsExpr *>{idx},
                                          /*is_mut=*/false));
    }

    RsExpr *offset = nullptr;
    {
      PushConversionKind push(*this, ConversionKind::Unboxed);
      auto *object = ConvertObject(expr->getArg(0));
      auto *ptr_type = ConvertPtrType(expr->getArg(0)->getType());
      auto *idx = ConvertSubscriptIndex(expr->getArg(1));
      offset = MethodCall(arena_.New<Cast>(object, ptr_type), "offset",
                          std::vector<RsExpr *>{idx}, /*is_mut=*/false);
    }

    auto *node = offset;
    if (is_inner_boxed) {
      node =
          Cat(arena_.New<Unary>(Unary::Op::Deref, node), Text(".as_pointer()"));
      if (!isObject()) {
        node = arena_.New<Cast>(
            node, Cat(Text("Ptr<"), Convert(expr->getType()), Text('>')));
      }
    }

    if (isAddrOf()) {
      computed_expr_type_ = ComputedExprType::FreshPointer;
    } else {
      node = arena_.New<Unary>(Unary::Op::Deref, node);
      SetValueFreshness(expr->getType());
    }
    return node;
  }
  default:
    return Converter::ConvertCXXOperatorCallExpr(expr);
  }
}

std::vector<RsExpr *>
ConverterRefCount::ConvertFunctionParameters(clang::FunctionDecl *decl) {
  PushConversionKind push(*this, ConversionKind::Unboxed);
  if (decl->isMain() && (decl->getNumParams() != 0U)) {
    return {Text(std::format("{}: i32",
                             GetNamedDeclAsString(decl->getParamDecl(0)))),
            Text(std::format("{}: Ptr<Ptr<u8>>",
                             GetNamedDeclAsString(decl->getParamDecl(1))))};
  }
  return Converter::ConvertFunctionParameters(decl);
}

RsExpr *ConverterRefCount::ConvertSubscriptIndex(clang::Expr *idx) {
  auto *node = ConvertRValue(idx);
  if (idx->getType()->isEnumeralType()) {
    return Cat(Parens(node), Text("as isize"));
  }
  return node;
}

RsExpr *ConverterRefCount::ConvertArraySubscript(clang::Expr *base,
                                                 clang::Expr *idx,
                                                 clang::QualType type) {
  if (auto *member =
          clang::dyn_cast<clang::MemberExpr>(base->IgnoreParenImpCasts())) {
    if (auto *fam = TryFlexibleArrayMember(member)) {
      auto *idx_node =
          arena_.New<Cast>(Parens(ConvertSubscriptIndex(idx)), Text("isize"));
      auto *node = MethodCall(fam, "offset", std::vector<RsExpr *>{idx_node},
                              /*is_mut=*/false);
      if (isAddrOf()) {
        computed_expr_type_ = ComputedExprType::FreshPointer;
        return node;
      }
      SetValueFreshness(type);
      return arena_.New<Unary>(Unary::Op::Deref, node);
    }
  }
  if (isAddrOf()) {
    bool is_inner_boxed = false;
    if (auto base_arr_ty = clang::dyn_cast<clang::ArrayType>(
            base->IgnoreImplicit()->getType().getTypePtr())) {
      is_inner_boxed = clang::isa<clang::ArrayType>(
          base_arr_ty->getElementType().getTypePtr());
    }

    RsExpr *node = nullptr;
    if (IsStringLiteralExpr(base)) {
      auto *base_node = ConvertExpr(base->IgnoreParens()->IgnoreImplicit());
      auto *idx_node = ConvertSubscriptIndex(idx);
      auto *literal = arena_.New<Call>(Text("Ptr::from_string_literal"),
                                       std::vector<RsExpr *>{base_node});
      node = MethodCall(literal, "offset", std::vector<RsExpr *>{idx_node},
                        /*is_mut=*/false);
    } else {
      auto *base_node = ConvertExpr(base->IgnoreImplicit());
      auto *ptr_type = ConvertPtrType(base->IgnoreImplicit()->getType());
      auto *idx_node = ConvertSubscriptIndex(idx);
      node = MethodCall(arena_.New<Cast>(base_node, ptr_type), "offset",
                        std::vector<RsExpr *>{idx_node}, /*is_mut=*/false);
    }

    if (is_inner_boxed) {
      node =
          Cat(arena_.New<Unary>(Unary::Op::Deref, node), Text(".as_pointer()"));
    }
    computed_expr_type_ = ComputedExprType::FreshPointer;
    return Parens(node, is_inner_boxed);
  }

  RsExpr *base_node = nullptr;
  if (isLValue() &&
      clang::isa<clang::ArraySubscriptExpr>(base->IgnoreImplicit())) {
    PushExprKind push(*this, ExprKind::RValue);
    base_node = ConvertExpr(base->IgnoreImplicit());
  } else {
    base_node = ConvertExpr(base->IgnoreImplicit());
  }
  if (clang::isa<clang::ArraySubscriptExpr>(base->IgnoreImplicit())) {
    base_node = isRValue()
                    ? static_cast<RsExpr *>(arena_.New<BorrowRead>(base_node))
                    : arena_.New<BorrowWrite>(base_node);
  }
  auto *idx_node = ConvertRValue(idx);
  SetValueFreshness(type);
  return arena_.New<Index>(base_node, idx_node);
}

RsExpr *
ConverterRefCount::ConvertPointerSubscript(clang::ArraySubscriptExpr *expr) {
  auto *base = expr->getBase();
  auto *idx = expr->getIdx();

  if (isLValue()) {
    return arena_.New<Unary>(Unary::Op::Deref, ConvertPointerOffset(base, idx));
  }

  if (isAddrOf()) {
    return ConvertPointerOffset(base, idx);
  }

  auto *offset = ConvertPointerOffset(base, idx);
  auto *node = arena_.New<Unary>(Unary::Op::Deref, offset);
  SetValueFreshness(expr->getType());
  return node;
}

RsExpr *ConverterRefCount::ConvertFunctionMain(
    const clang::FunctionDecl *decl,
    const std::string_view main_function_name) {
  if (decl->getNumParams() != 0U) {
    return Text(std::format(R"(
pub fn main() {{
    let argv: Vec<Value<Vec<u8>>> = ::std::env::args()
        .map(|x| Rc::new(RefCell::new(x.as_bytes().to_vec())))
        .collect();
    let mut argv: Value<Vec<Ptr<u8>>> = Rc::new(RefCell::new(
        argv.iter().map(|x| {{ x.borrow_mut().push(0); x.as_pointer() }}).collect(),
    ));
    (*argv.borrow_mut()).push(Ptr::null());
    libcc2rs::exit_refcount({}(::std::env::args().len() as i32,
                                argv.as_pointer()));
}})",
                            main_function_name));
  }
  return Text(std::format("pub fn main() {{ libcc2rs::exit_refcount({}()); }}",
                          main_function_name));
}

RsExpr *ConverterRefCount::ConvertAddrOf(clang::Expr *expr,
                                         clang::QualType pointer_type) {
  if (const auto *arr = ctx_.getAsArrayType(expr->getType())) {
    PushConversionKind push(*this, ConversionKind::Unboxed);
    auto *node = ConvertPointer(expr);
    return arena_.New<Cast>(
        node, Cat(Text("Ptr<"), Convert(arr->getElementType()), Text('>')));
  }
  return ConvertPointer(expr);
}

RsExpr *ConverterRefCount::ConvertDeref(clang::Expr *expr) {
  auto pointee_type = expr->getType()->getPointeeType();

  if (isLValue()) {
    return arena_.New<Unary>(Unary::Op::Deref, ConvertExpr(expr));
  }

  RsExpr *node = nullptr;
  if (isAddrOf()) {
    node = ConvertExpr(expr);
  } else {
    auto *inner = ConvertExpr(expr);
    node = arena_.New<Unary>(Unary::Op::Deref, inner);
    SetValueFreshness(pointee_type);
  }

  if (isObject()) {
    if (IsBoxedType(pointee_type)) {
      computed_expr_type_ = ComputedExprType::FreshPointer;
      return Cat(node, Text(".to_strong().as_pointer()"));
    }
  }
  return node;
}

RsExpr *ConverterRefCount::ConvertArrow(clang::Expr *expr) {
  auto *op = clang::dyn_cast<clang::CXXOperatorCallExpr>(expr);
  bool is_overloaded_arrow =
      op && op->getOperator() == clang::OverloadedOperatorKind::OO_Arrow;

  if (!is_overloaded_arrow) {
    auto *ptr = ConvertExpr(expr);
    auto *node = arena_.New<Unary>(Unary::Op::Deref, ptr);
    SetValueFreshness(expr->getType()->getPointeeType());
    return node;
  }

  if (GetStrongestIteratorCategory(op->getArg(0)->getType()) ==
      IteratorCategory::Bidirectional) {
    return ConvertExpr(op->getArg(0));
  }

  return ConvertExpr(expr);
}

RsExpr *ConverterRefCount::AccessLValueObject(clang::MemberExpr *member) {
  auto *method = clang::dyn_cast<clang::CXXMethodDecl>(member->getMemberDecl());
  auto *object = member->getBase();

  bool is_mut = method && !method->isConst();
  if (member->isArrow()) {
    auto *op =
        clang::dyn_cast<clang::CXXOperatorCallExpr>(object->IgnoreImplicit());
    if (op && GetStrongestIteratorCategory(op->getArg(0)->getType()) ==
                  IteratorCategory::Bidirectional) {
      return ConvertRValue(op->getArg(0));
    }
    auto *node = is_mut ? ConvertLValue(object) : ConvertRValue(object);
    return arena_.New<Unary>(Unary::Op::Deref, node);
  }
  return is_mut ? ConvertLValue(object) : ConvertRValue(object);
}

RsExpr *ConverterRefCount::emplace_back_plugin_construct_arg(
    clang::QualType elem_type, clang::CXXConstructExpr *ctor) {
  PushUnboxedIfSimple push(*this, "Vec<%>", elem_type);
  return ConvertVarInit(elem_type, ctor);
}

RsExpr *
ConverterRefCount::emplace_back_emit_push_open(clang::CXXMemberCallExpr *call) {
  auto *obj = GetCallObject(call);
  auto obj_type = obj->getType().getNonReferenceType();
  if (obj_type->isPointerType()) {
    obj_type = obj_type->getPointeeType();
  }
  auto *object = ConvertObject(obj);
  auto *type_node = Convert(obj_type.getNonReferenceType());
  return Cat(object, Text(".with_mut("),
             arena_.New<Closure>("__v", Cat(Text("&mut"), type_node),
                                 Text("__v.push(")));
}

RsExpr *ConverterRefCount::emplace_back_emit_push_close(
    clang::CXXMemberCallExpr *call) {
  return Text("))");
}

bool ConverterRefCount::IsReferenceType(const clang::Expr *expr) const {
  if (Converter::IsReferenceType(expr)) {
    return true;
  }
  if (auto *call =
          clang::dyn_cast<clang::CXXOperatorCallExpr>(expr->IgnoreCasts())) {
    return GetReturnTypeOfFunction(call)->isReferenceType();
  }
  return false;
}

RsExpr *ConverterRefCount::ConvertMappedMethodCall(
    clang::Expr *expr, const TranslationRule::MethodCallFragment &mc,
    clang::Expr **args, unsigned num_args, TempMaterializationCtx *ctx) {
  auto receiver_ph = mc.getReceiverPlaceholder();
  if (!receiver_ph || receiver_ph->access == TranslationRule::Access::kRead) {
    return Converter::ConvertMappedMethodCall(expr, mc, args, num_args, ctx);
  }

  auto arg_idx = receiver_ph->n;
  auto *arg = BuildUnifiedArgs(expr, args, num_args)[arg_idx];

  if (!arg->getType()->isPointerType() && !IsReferenceType(arg)) {
    PushExprKind push(*this, ExprKind::LValue);
    auto *receiver = ConvertIRFragment(mc.receiver, expr, args, num_args, ctx);
    auto *body = ConvertIRFragment(mc.body, expr, args, num_args, ctx);
    auto *node = Cat(receiver, body);
    if (auto *ptr = receiver->TakePtr(Text("__v"))) {
      return arena_.New<PtrWith>(ptr, true,
                                 arena_.New<Closure>("__v", nullptr, node));
    }
    return node;
  }

  auto param_type = Mapper::GetParamType(GetCalleeOrExpr(expr), arg_idx);

  if (arg->getType()->isPointerType()) {
    auto *ptr = ConvertPointer(arg);
    auto *body = ConvertIRFragment(mc.body, expr, args, num_args, ctx);
    return arena_.New<PtrWith>(
        ptr, true,
        arena_.New<Closure>("__v", Text(param_type), Cat(Text("__v"), body)));
  }

  auto *receiver =
      ConvertIRFragment(mc.receiver, expr, args, num_args, ctx)->IgnoreParens();
  auto *receiver_ptr = receiver->Pointer();
  assert(receiver_ptr && "receiver is not a dereference");
  auto *body = ConvertIRFragment(mc.body, expr, args, num_args, ctx);

  if (PointeeIsBoxed(receiver->expr)) {
    return arena_.New<PtrWith>(
        receiver_ptr, true,
        arena_.New<Closure>(
            "__v", Cat(Text("&mut Value<"), Convert(arg->getType()), Text('>')),
            Cat(arena_.New<BorrowWrite>(Text("__v")), body)));
  }

  return arena_.New<PtrWith>(
      receiver_ptr, true,
      arena_.New<Closure>("__v", Text(param_type), Cat(Text("__v"), body)));
}

RsExpr *ConverterRefCount::ConvertPointeeType(clang::QualType ptr_type) {
  assert(!ptr_type.isNull() && ptr_type->isPointerType());
  PushConversionKind push(*this, ConversionKind::Unboxed);
  auto pointee = ptr_type->getPointeeType();
  if (!pointee->isRecordType()) {
    return Convert(pointee);
  }

  // Pointee of a pointer to incomplete type is an incomplete type that does
  // not have a translation rule, so converting the pointee alone is not
  // enough.
  auto str = RenderType(ptr_type);
  Unwrap(str, "PtrDyn<", ">");
  Unwrap(str, "Ptr<", ">");
  return Text(std::move(str));
}

} // namespace cpp2rust
