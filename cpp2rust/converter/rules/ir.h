#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <llvm/Support/JSON.h>

#include <cassert>
#include <filesystem>
#include <optional>
#include <string>
#include <unordered_map>

namespace cpp2rust::Ir {

static inline constexpr unsigned kMaxGenerics = 9;

template <typename Expr, typename Type> struct Rules {
  std::unordered_map<std::string, Expr> exprs;
  std::unordered_map<std::string, Type> types;
};

std::optional<llvm::json::Object> ReadJSON(const std::filesystem::path &path,
                                           bool required);

template <typename Expr, typename Type, typename ParseExpr, typename ParseType>
void LoadJSON(Rules<Expr, Type> &rules, const std::filesystem::path &path,
              bool required, ParseExpr parse_expr, ParseType parse_type) {
  auto root = ReadJSON(path, required);
  if (!root) {
    return;
  }
  for (auto &[key, value] : *root) {
    const auto *obj = value.getAsObject();
    assert(obj && "rule entry must be an object");
    auto name = key.str();
    if (name[0] == 'f') {
      rules.exprs[std::move(name)] = parse_expr(*obj);
    } else if (name[0] == 't') {
      rules.types[std::move(name)] = parse_type(*obj);
    }
  }
}

} // namespace cpp2rust::Ir
