#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/Type.h>
#include <llvm/Support/Casting.h>
#include <llvm/Support/JSON.h>

#include <cassert>
#include <cstddef>
#include <filesystem>
#include <functional>
#include <memory>
#include <string>
#include <type_traits>
#include <utility>
#include <vector>

#include "converter/rules/ir.h"

namespace cpp2rust::IrSrc {

struct Node;
using NodePtr = std::unique_ptr<Node>;

struct Node {
  enum class Kind {
    kParam,
    kValue,
    kBuiltin,
    kRecord,
    kEnum,
    kTypedef,
    kConst,
    kVolatile,
    kPointer,
    kLRef,
    kRRef,
    kArray,
    kFunctionType,
    kOpaque,
    kFunction,
    kDecl,
    kMacro,
    kUnary,
    kArrow,
    kPackParams,
  };

  const Kind kind;
  std::string name;
  clang::QualType type;

  explicit Node(Kind kind, std::string name = {})
      : kind(kind), name(std::move(name)) {}
  Node(const Node &) = delete;
  Node &operator=(const Node &) = delete;
  virtual ~Node() = default;

  bool operator==(const Node &other) const;
  bool shallowEquals(const Node &other) const;
  std::string indexKey() const;
  unsigned specificity() const;
  void forEachParam(const std::function<void(unsigned)> &fn) const;
  bool hasParam(unsigned n) const;
  std::string str() const;
};

struct ParamNode : Node {
  unsigned param;

  explicit ParamNode(unsigned param) : Node(Kind::kParam), param(param) {}
  static bool classof(const Node *node) { return node->kind == Kind::kParam; }
};

struct QualNode : Node {
  NodePtr operand;

  QualNode(Kind kind, NodePtr operand)
      : Node(kind), operand(std::move(operand)) {}
  static bool classof(const Node *node) {
    return node->kind == Kind::kConst || node->kind == Kind::kVolatile;
  }
};

struct PointerNode : Node {
  NodePtr pointee;

  PointerNode(Kind kind, NodePtr pointee)
      : Node(kind), pointee(std::move(pointee)) {}
  static bool classof(const Node *node) {
    return node->kind == Kind::kPointer || node->kind == Kind::kLRef ||
           node->kind == Kind::kRRef;
  }
};

struct ArrayNode : Node {
  NodePtr element;
  NodePtr size;

  explicit ArrayNode(NodePtr element)
      : Node(Kind::kArray), element(std::move(element)) {}
  static bool classof(const Node *node) { return node->kind == Kind::kArray; }
};

struct RecordNode : Node {
  NodePtr class_;
  std::vector<NodePtr> args;

  explicit RecordNode(std::string name)
      : Node(Kind::kRecord, std::move(name)) {}
  static bool classof(const Node *node) { return node->kind == Kind::kRecord; }
};

struct DeclNode : Node {
  NodePtr class_;

  explicit DeclNode(std::string name) : Node(Kind::kDecl, std::move(name)) {}
  static bool classof(const Node *node) { return node->kind == Kind::kDecl; }
};

struct FunctionNode : Node {
  NodePtr class_;
  NodePtr return_type;
  std::vector<NodePtr> params;
  bool variadic = false;
  bool is_const = false;
  bool is_volatile = false;
  std::string ref;

  explicit FunctionNode(Kind kind, std::string name = {})
      : Node(kind, std::move(name)) {}
  static bool classof(const Node *node) {
    return node->kind == Kind::kFunction || node->kind == Kind::kFunctionType;
  }
};

struct UnaryNode : Node {
  NodePtr operand;

  UnaryNode(std::string op, NodePtr operand)
      : Node(Kind::kUnary, std::move(op)), operand(std::move(operand)) {}
  static bool classof(const Node *node) { return node->kind == Kind::kUnary; }
};

struct ArrowNode : Node {
  NodePtr object;
  NodePtr member;

  ArrowNode(NodePtr object, NodePtr member)
      : Node(Kind::kArrow), object(std::move(object)),
        member(std::move(member)) {}
  static bool classof(const Node *node) { return node->kind == Kind::kArrow; }
};

template <typename A, typename B, typename Fn>
bool forEachField(A &a, B &b, Fn fn) {
  using Kind = Node::Kind;
  using llvm::cast;
  assert(a.kind == b.kind);
  switch (a.kind) {
  case Kind::kConst:
  case Kind::kVolatile:
    return fn("operand", cast<QualNode>(a).operand, cast<QualNode>(b).operand);
  case Kind::kPointer:
  case Kind::kLRef:
  case Kind::kRRef:
    return fn("pointee", cast<PointerNode>(a).pointee,
              cast<PointerNode>(b).pointee);
  case Kind::kArray:
    return fn("element", cast<ArrayNode>(a).element,
              cast<ArrayNode>(b).element) &&
           fn("size", cast<ArrayNode>(a).size, cast<ArrayNode>(b).size);
  case Kind::kRecord:
    return fn("class", cast<RecordNode>(a).class_,
              cast<RecordNode>(b).class_) &&
           fn("args", cast<RecordNode>(a).args, cast<RecordNode>(b).args);
  case Kind::kDecl:
    return fn("class", cast<DeclNode>(a).class_, cast<DeclNode>(b).class_);
  case Kind::kFunction:
  case Kind::kFunctionType:
    return fn("class", cast<FunctionNode>(a).class_,
              cast<FunctionNode>(b).class_) &&
           fn("return_type", cast<FunctionNode>(a).return_type,
              cast<FunctionNode>(b).return_type) &&
           fn("params", cast<FunctionNode>(a).params,
              cast<FunctionNode>(b).params);
  case Kind::kUnary:
    return fn("operand", cast<UnaryNode>(a).operand,
              cast<UnaryNode>(b).operand);
  case Kind::kArrow:
    return fn("object", cast<ArrowNode>(a).object, cast<ArrowNode>(b).object) &&
           fn("member", cast<ArrowNode>(a).member, cast<ArrowNode>(b).member);
  default:
    return true;
  }
}

template <typename T>
inline constexpr bool kIsNodeList =
    std::is_same_v<std::remove_cvref_t<T>, std::vector<NodePtr>>;

template <typename A, typename B, typename Fn>
bool zipChildren(A &a, B &b, Fn fn) {
  return forEachField(a, b, [&](const char *, auto &x, auto &y) {
    if constexpr (kIsNodeList<decltype(x)>) {
      if (x.size() != y.size()) {
        return false;
      }
      for (std::size_t i = 0; i < x.size(); ++i) {
        if (!fn(x[i], y[i])) {
          return false;
        }
      }
      return true;
    } else {
      if (!x || !y) {
        return !x && !y;
      }
      return fn(x, y);
    }
  });
}

llvm::json::Value ToJSON(const Node &node);

struct InitTypeLocation {
  unsigned depth = -1u;
  unsigned index = -1u;

  bool valid() const { return depth != -1u; }
};

struct ExprRule {
  NodePtr ir;
  unsigned num_params = 0;
  InitTypeLocation init_type;

  void dump() const;
};

struct TypeRule {
  NodePtr ir;
  unsigned num_params = 0;

  void dump() const;
};

using Rules = Ir::Rules<ExprRule, TypeRule>;

Rules Load(const std::filesystem::path &dir);

} // namespace cpp2rust::IrSrc
