#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/Type.h>
#include <llvm/Support/JSON.h>

#include <filesystem>
#include <functional>
#include <string>
#include <vector>

#include "converter/rules/ir.h"

namespace cpp2rust::IrSrc {

struct Node {
  enum class Kind {
    kNone,
    kParam,
    kValue,
    kBuiltin,
    kRecord,
    kEnum,
    kTypedef,
    kPointer,
    kLRef,
    kRRef,
    kArray,
    kIncompleteArray,
    kFunctionType,
    kPack,
    kOpaque,
    kFunction,
    kDecl,
    kMacro,
    kUnary,
    kArrow,
    kPackParams,
  };

  Kind kind = Kind::kNone;
  std::string name;
  unsigned param = 0;
  bool is_const = false;
  bool is_volatile = false;
  bool variadic = false;
  std::string ref;
  std::vector<Node> children;
  clang::QualType type;

  bool operator==(const Node &other) const;
  unsigned specificity() const;
  void forEachParam(const std::function<void(unsigned)> &fn) const;
  std::string str() const;
};

llvm::json::Value ToJSON(const Node &node);

struct InitTypeLocation {
  unsigned depth = -1u;
  unsigned index = -1u;

  bool valid() const { return depth != -1u; }
};

struct ExprRule {
  Node ir;
  InitTypeLocation init_type;

  void dump() const;
};

struct TypeRule {
  Node ir;

  void dump() const;
};

using Rules = Ir::Rules<ExprRule, TypeRule>;

Rules Load(const std::filesystem::path &dir);

} // namespace cpp2rust::IrSrc
