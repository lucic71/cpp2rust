// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "rs_expr.h"

#include <clang/AST/Expr.h>
#include <llvm/Support/Casting.h>
#include <llvm/Support/raw_ostream.h>

namespace cpp2rust {

Cast::Cast(RsExpr *expr, RsExpr *type, clang::QualType pointee)
    : RsExpr(Kind::Cast), expr(expr), type(type) {
  if (pointee.isNull()) {
    return;
  }
  auto wanted = pointee.getCanonicalType().getUnqualifiedType();
  expr->ForEachTail([&](RsExpr *operand) {
    if (auto *ptr_view = llvm::dyn_cast<PtrView>(operand)) {
      ptr_view->element =
          wanted != ptr_view->view_type.getCanonicalType().getUnqualifiedType();
      return;
    }
    if (auto *field_ptr = llvm::dyn_cast<FieldPtr>(operand);
        field_ptr && field_ptr->container) {
      field_ptr->element =
          wanted !=
          field_ptr->field_type.getCanonicalType().getUnqualifiedType();
    }
  });
}

std::string Binary::print() const {
  return lhs->print() + ' ' +
         std::string(clang::BinaryOperator::getOpcodeStr(op)) + ' ' +
         rhs->print() + ' ';
}

RsExpr *RsExpr::IgnoreParens() {
  auto *node = this;
  while (auto *delim = llvm::dyn_cast<Delim>(node)) {
    if (delim->open != '(' || delim->close != ')') {
      break;
    }
    node = delim->inner;
  }
  return node;
}

PtrWith *RsExpr::TakeWithFrom(RsExpr *&slot) {
  if (auto *with = llvm::dyn_cast<PtrWith>(slot)) {
    slot = llvm::cast<Closure>(with->closure)->body;
    return with;
  }
  return slot->TakeWith();
}

PtrWith *RsExpr::TakeWith() {
  auto **slot = ChainSlot();
  return slot != nullptr ? TakeWithFrom(*slot) : nullptr;
}

RsExpr *RsExpr::TakePtr(RsExpr *replacement) {
  auto **slot = ChainSlot();
  if (slot == nullptr) {
    return nullptr;
  }
  if (auto *ptr = (*slot)->Pointer()) {
    *slot = replacement;
    return ptr;
  }
  return (*slot)->TakePtr(replacement);
}

PtrWith *RsExpr::FindWith() {
  if (auto *with = llvm::dyn_cast<PtrWith>(this)) {
    return with;
  }
  auto **slot = ChainSlot();
  return slot != nullptr ? (*slot)->FindWith() : nullptr;
}

bool ReadsClosureParam(RsExpr *node) {
  if (llvm::isa<Closure>(node)) {
    return false;
  }
  if (llvm::isa<ClosureParam>(node)) {
    return true;
  }
  bool used = false;
  node->ForEachChild(
      [&](RsExpr *&child) { used = used || ReadsClosureParam(child); });
  return used;
}

bool RsExpr::Any(llvm::function_ref<bool(RsExpr *)> pred) {
  if (pred(this)) {
    return true;
  }
  bool found = false;
  ForEachChild([&](RsExpr *&child) { found = found || child->Any(pred); });
  return found;
}

const char *KindName(RsExpr::Kind kind) {
  switch (kind) {
  case RsExpr::Kind::Verbatim:
    return "Verbatim";
  case RsExpr::Kind::ClosureParam:
    return "ClosureParam";
  case RsExpr::Kind::Concat:
    return "Concat";
  case RsExpr::Kind::Delim:
    return "Delim";
  case RsExpr::Kind::Unary:
    return "Unary";
  case RsExpr::Kind::Cast:
    return "Cast";
  case RsExpr::Kind::Clone:
    return "Clone";
  case RsExpr::Kind::Take:
    return "Take";
  case RsExpr::Kind::Call:
    return "Call";
  case RsExpr::Kind::Closure:
    return "Closure";
  case RsExpr::Kind::Conditional:
    return "Conditional";
  case RsExpr::Kind::Binary:
    return "Binary";
  case RsExpr::Kind::Literal:
    return "Literal";
  case RsExpr::Kind::Let:
    return "Let";
  case RsExpr::Kind::Return:
    return "Return";
  case RsExpr::Kind::If:
    return "If";
  case RsExpr::Kind::Loop:
    return "Loop";
  case RsExpr::Kind::Break:
    return "Break";
  case RsExpr::Kind::Continue:
    return "Continue";
  case RsExpr::Kind::Assign:
    return "Assign";
  case RsExpr::Kind::CompoundAssign:
    return "CompoundAssign";
  case RsExpr::Kind::Fn:
    return "Fn";
  case RsExpr::Kind::Trait:
    return "Trait";
  case RsExpr::Kind::Impl:
    return "Impl";
  case RsExpr::Kind::Field:
    return "Field";
  case RsExpr::Kind::Index:
    return "Index";
  case RsExpr::Kind::FieldPtr:
    return "FieldPtr";
  case RsExpr::Kind::PtrView:
    return "PtrView";
  case RsExpr::Kind::BitField:
    return "BitField";
  case RsExpr::Kind::BorrowRead:
    return "BorrowRead";
  case RsExpr::Kind::BorrowWrite:
    return "BorrowWrite";
  case RsExpr::Kind::PtrRead:
    return "PtrRead";
  case RsExpr::Kind::PtrWrite:
    return "PtrWrite";
  case RsExpr::Kind::PtrWith:
    return "PtrWith";
  }
  return "Unknown";
}

namespace {

void DumpHeader(const RsExpr *node, llvm::raw_ostream &os, unsigned depth,
                const std::string &details = {}) {
  os.indent(depth * 2) << KindName(node->kind);
  if (!details.empty()) {
    os << ' ' << details;
  }
  os << '\n';
}

void DumpChildren(RsExpr *node, llvm::raw_ostream &os, unsigned depth) {
  node->ForEachChild([&](RsExpr *&child) { child->dump(os, depth + 1); });
}

} // namespace

bool RsExpr::ContainsBorrow() {
  return Any([](RsExpr *node) {
    return llvm::isa<BorrowRead>(node) || llvm::isa<BorrowWrite>(node);
  });
}

void RsExpr::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth);
  DumpChildren(this, os, depth);
}

void Verbatim::dump(llvm::raw_ostream &os, unsigned depth) {
  std::string line = text;
  for (auto &ch : line) {
    if (ch == '\n') {
      ch = ' ';
    }
  }
  if (line.size() > 60) {
    line.resize(60);
    line += "...";
  }
  DumpHeader(this, os, depth, '"' + line + '"');
}

void ClosureParam::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, deref ? "deref" : "");
}

void Concat::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth);
  DumpChildren(this, os, depth);
}

void Delim::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, std::string(1, open) + std::string(1, close));
  DumpChildren(this, os, depth);
}

void Unary::dump(llvm::raw_ostream &os, unsigned depth) {
  const char *name = "";
  switch (op) {
  case Op::Deref:
    name = "deref";
    break;
  case Op::Not:
    name = "not";
    break;
  case Op::Neg:
    name = "neg";
    break;
  }
  DumpHeader(this, os, depth, name);
  DumpChildren(this, os, depth);
}

void Cast::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth);
  DumpChildren(this, os, depth);
}

void Clone::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth);
  DumpChildren(this, os, depth);
}

void Take::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth);
  DumpChildren(this, os, depth);
}

void Call::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, is_mut ? "mut" : "");
  DumpChildren(this, os, depth);
}

void Closure::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, '|' + param + '|');
  DumpChildren(this, os, depth);
}

void Conditional::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth);
  DumpChildren(this, os, depth);
}

void Let::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, (is_mut ? "mut " : "") + name);
  DumpChildren(this, os, depth);
}

void Return::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth);
  DumpChildren(this, os, depth);
}

void If::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth);
  DumpChildren(this, os, depth);
}

void Loop::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, label.empty() ? keyword : label + ": " + keyword);
  DumpChildren(this, os, depth);
}

void Break::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, label);
}

void Continue::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, label);
}

void Binary::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth,
             std::string(clang::BinaryOperator::getOpcodeStr(op)));
  DumpChildren(this, os, depth);
}

void Literal::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, text);
}

void Fn::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, name);
  DumpChildren(this, os, depth);
}

void Trait::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, name);
  DumpChildren(this, os, depth);
}

void Impl::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, trait_name);
  DumpChildren(this, os, depth);
}

void Field::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, '.' + member);
  DumpChildren(this, os, depth);
}

void BitField::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, '.' + member + "(): " + type_name);
  DumpChildren(this, os, depth);
}

void Index::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth);
  DumpChildren(this, os, depth);
}

void PtrView::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth,
             view_type.getAsString() + (element ? " element" : ""));
  DumpChildren(this, os, depth);
}

void FieldPtr::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth,
             type_name + "::" + field + ": " + field_type.getAsString() + " @" +
                 std::to_string(offset) + (element ? " element" : ""));
  DumpChildren(this, os, depth);
}

void BorrowRead::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth);
  DumpChildren(this, os, depth);
}

void BorrowWrite::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth);
  DumpChildren(this, os, depth);
}

void PtrRead::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth);
  DumpChildren(this, os, depth);
}

void PtrWrite::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth);
  DumpChildren(this, os, depth);
}

void PtrWith::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, is_mut ? "mut" : "");
  DumpChildren(this, os, depth);
}

void Assign::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth);
  DumpChildren(this, os, depth);
}

void CompoundAssign::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, op);
  DumpChildren(this, os, depth);
}

} // namespace cpp2rust
