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

PtrWith *RsExpr::TakeWithFrom(RsExpr *&slot) {
  if (auto *with = llvm::dyn_cast<PtrWith>(slot)) {
    slot = llvm::cast<Closure>(with->closure)->body;
    return with;
  }
  return slot->TakeWith();
}

} // namespace cpp2rust
