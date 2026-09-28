// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/ir.h"

#include <llvm/Support/MemoryBuffer.h>
#include <llvm/Support/raw_ostream.h>

#include <cstdlib>

namespace cpp2rust::Ir {

std::optional<llvm::json::Object> ReadJSON(const std::filesystem::path &path,
                                           bool required) {
  auto buf = llvm::MemoryBuffer::getFile(path.string());
  if (!buf) {
    if (required) {
      llvm::errs() << "ERROR: missing " << path << '\n';
      std::exit(EXIT_FAILURE);
    }
    return std::nullopt;
  }

  auto parsed = llvm::json::parse((*buf)->getBuffer());
  if (!parsed) {
    llvm::errs() << "ERROR: failed to parse " << path << ": "
                 << llvm::toString(parsed.takeError()) << '\n';
    std::exit(EXIT_FAILURE);
  }

  auto *root = parsed->getAsObject();
  if (!root) {
    llvm::errs() << "ERROR: " << path << " is not a JSON object\n";
    std::exit(EXIT_FAILURE);
  }
  return std::move(*root);
}

} // namespace cpp2rust::Ir
