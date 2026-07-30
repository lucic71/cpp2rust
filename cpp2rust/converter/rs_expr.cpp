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

RsExpr *DerefOperand(RsExpr *node) {
  auto *deref = llvm::dyn_cast<Unary>(node->IgnoreParens());
  if (!deref || deref->op != Unary::Op::Deref) {
    return nullptr;
  }
  return deref->operand;
}

} // namespace cpp2rust
