// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "logging.h"

namespace cpp2rust {
namespace {
llvm::raw_ostream *verrs_ = &llvm::nulls();
llvm::raw_ostream *vouts_ = &llvm::nulls();
} // namespace

void SetVerbose(bool verbose) {
  verrs_ = verbose ? &llvm::errs() : &llvm::nulls();
  vouts_ = verbose ? &llvm::outs() : &llvm::nulls();
}

llvm::raw_ostream &verrs() { return *verrs_; }
llvm::raw_ostream &vouts() { return *vouts_; }

} // namespace cpp2rust
