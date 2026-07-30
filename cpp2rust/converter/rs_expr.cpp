// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "rs_expr.h"

#include <llvm/Support/Casting.h>

namespace cpp2rust {

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

namespace {

const char *ReceiverText(WithReceiver receiver) {
  switch (receiver) {
  case WithReceiver::Direct:
    return "__v";
  case WithReceiver::Borrow:
    return "(*__v.borrow_mut())";
  }
  std::unreachable();
}

} // namespace

RsExpr *MakeAssign(RsArena &arena, RsExpr *lhs, RsExpr *rhs) {
  if (auto *place = llvm::dyn_cast<Unary>(lhs->IgnoreParens());
      place && place->op == Unary::Op::Deref) {
    return arena.New<Concat>(
        std::vector<RsExpr *>{place->operand, arena.New<Verbatim>(".write("),
                              rhs, arena.New<Verbatim>(")")});
  }
  return arena.New<Concat>(
      std::vector<RsExpr *>{lhs, arena.New<Verbatim>("="), rhs});
}

RsExpr *MakeCompoundAssign(RsArena &arena, RsExpr *lhs, std::string_view op,
                           RsExpr *rhs) {
  if (auto *place = llvm::dyn_cast<Unary>(lhs->IgnoreParens());
      place && place->op == Unary::Op::Deref) {
    op.remove_suffix(1); // remove '='
    auto *block = arena.New<Concat>(std::vector<RsExpr *>{
        arena.New<Verbatim>("let _ptr ="), place->operand,
        arena.New<Verbatim>(".clone()"), arena.New<Verbatim>(";"),
        arena.New<Verbatim>("_ptr.write(_ptr.read()"),
        arena.New<Verbatim>(std::string(op)), rhs, arena.New<Verbatim>(")")});
    return arena.New<Delim>('{', '}', block);
  }
  return arena.New<Concat>(
      std::vector<RsExpr *>{lhs, arena.New<Verbatim>(std::string(op)), rhs});
}

RsExpr *MakeMethodCall(RsArena &arena, RsExpr *lhs, RsExpr *param_type,
                       WithReceiver receiver, RsExpr *call) {
  auto *place = llvm::dyn_cast<Unary>(lhs->IgnoreParens());
  if (!place || place->op != Unary::Op::Deref) {
    return arena.New<Concat>(std::vector<RsExpr *>{lhs, call});
  }

  std::vector<RsExpr *> parts{place->operand,
                              arena.New<Verbatim>(".with_mut(|__v")};
  if (param_type) {
    parts.push_back(arena.New<Verbatim>(":"));
    parts.push_back(param_type);
  }
  parts.push_back(arena.New<Verbatim>("|"));
  parts.push_back(arena.New<Verbatim>(ReceiverText(receiver)));
  parts.push_back(call);
  parts.push_back(arena.New<Verbatim>(")"));
  return arena.New<Concat>(std::move(parts));
}

} // namespace cpp2rust
