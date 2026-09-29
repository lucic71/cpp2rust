#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/ASTContext.h>
#include <clang/AST/Decl.h>
#include <clang/AST/Expr.h>
#include <clang/AST/Type.h>
#include <llvm/Support/JSON.h>

#include <cstddef>
#include <filesystem>
#include <functional>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "converter/rules/ir.h"

namespace cpp2rust::IrSrc {

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

  Kind kind = Kind::kOpaque;
  std::string name;
  unsigned param = 0;
  bool is_const = false;
  bool is_volatile = false;
  bool variadic = false;
  std::string ref;

  std::shared_ptr<Node> class_;
  std::shared_ptr<Node> return_type;
  std::vector<Node> params;
  std::vector<Node> args;
  std::shared_ptr<Node> pointee;
  std::shared_ptr<Node> element;
  std::shared_ptr<Node> size;
  std::shared_ptr<Node> operand;
  std::shared_ptr<Node> object;
  std::shared_ptr<Node> member;

  clang::QualType type;

  bool operator==(const Node &other) const;
  bool shallowEquals(const Node &other) const;
  std::string indexKey() const;
  unsigned specificity() const;
  void forEachParam(const std::function<void(unsigned)> &fn) const;
  std::string str() const;

  template <typename A, typename B, typename Fn>
  static bool zipChildren(A &a, B &b, Fn fn);
};

inline constexpr std::pair<const char *, std::shared_ptr<Node> Node::*>
    kNodeFields[] = {
        {"class", &Node::class_},    {"return_type", &Node::return_type},
        {"pointee", &Node::pointee}, {"element", &Node::element},
        {"size", &Node::size},       {"operand", &Node::operand},
        {"object", &Node::object},   {"member", &Node::member},
};

inline constexpr std::pair<const char *, std::vector<Node> Node::*>
    kNodeLists[] = {
        {"params", &Node::params},
        {"args", &Node::args},
};

template <typename A, typename B, typename Fn>
bool Node::zipChildren(A &a, B &b, Fn fn) {
  for (const auto &entry : kNodeFields) {
    auto &x = a.*entry.second;
    auto &y = b.*entry.second;
    if (!x || !y) {
      if (x || y) {
        return false;
      }
      continue;
    }
    if (!fn(*x, *y)) {
      return false;
    }
  }
  for (const auto &entry : kNodeLists) {
    auto &x = a.*entry.second;
    auto &y = b.*entry.second;
    if (x.size() != y.size()) {
      return false;
    }
    for (std::size_t i = 0; i < x.size(); ++i) {
      if (!fn(x[i], y[i])) {
        return false;
      }
    }
  }
  return true;
}

Node Make(Node::Kind kind, std::string name = {});
std::shared_ptr<Node> Share(Node node);

llvm::json::Value ToJSON(const Node &node);

class Builder {
public:
  explicit Builder(clang::ASTContext &ctx) : ctx_(ctx) {}

  std::function<std::optional<unsigned>(const clang::Decl *)> stand_in;
  bool keep_builtin_typedef = false;
  std::function<bool(clang::QualType pointee)> keep_pointee_sugar;

  Node FromType(clang::QualType type);
  Node FromDecl(const clang::NamedDecl *decl);
  std::optional<Node> FromExpr(const clang::Expr *expr);

private:
  clang::ASTContext &ctx_;

  Node fromType(clang::QualType type, bool top);
  Node fromCanonical(clang::QualType canonical);
  Node fromTemplateArg(const clang::TemplateArgument &arg);
  Node fromRecord(const clang::RecordDecl *decl);
  std::shared_ptr<Node> classOf(const clang::Decl *decl);
};

struct InitTypeLocation {
  unsigned depth = -1u;
  unsigned index = -1u;

  bool valid() const { return depth != -1u; }
};

struct ExprRule {
  Node ir;
  unsigned num_params = 0;
  InitTypeLocation init_type;

  void dump() const;
};

struct TypeRule {
  Node ir;
  unsigned num_params = 0;

  void dump() const;
};

using Rules = Ir::Rules<ExprRule, TypeRule>;

Rules Load(const std::filesystem::path &dir);

} // namespace cpp2rust::IrSrc
