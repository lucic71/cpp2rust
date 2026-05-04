// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#pragma once

#include <llvm/Support/JSON.h>

#include <filesystem>

namespace cpp2rust::RuleSrcParser {

// Emits one entry per f<n>/t<n>: { "to_string": <Mapper::ToString> }.
// Returns false if clang errored or any f<n> body shape went unmatched.
bool Extract(const std::filesystem::path &src_path, llvm::json::Object &out);

} // namespace cpp2rust::RuleSrcParser
