#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <llvm/ADT/STLFunctionalExtras.h>

#include <cstdint>
#include <memory>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

namespace clang {
class Expr;
}

namespace cpp2rust {

struct RsExpr {
  enum class Kind : uint8_t {
    Verbatim,
    Concat,
    Delim,
    Unary,
    Assign,
    CompoundAssign,
    MethodCall,
  };

  explicit RsExpr(Kind kind) : kind(kind) {}
  virtual ~RsExpr() = default;

  virtual std::string print() const = 0;

  virtual void ForEachChild(llvm::function_ref<void(RsExpr *&)>) {}

  RsExpr *IgnoreParens();

  virtual RsExpr *Pointer() { return nullptr; }

  Kind kind;
  const clang::Expr *expr = nullptr;
};

struct Verbatim : RsExpr {
  explicit Verbatim(std::string text)
      : RsExpr(Kind::Verbatim), text(std::move(text)) {}

  static bool classof(const RsExpr *expr) {
    return expr->kind == Kind::Verbatim;
  }

  std::string print() const override { return text; }

  std::string text;
};

struct Concat : RsExpr {
  explicit Concat(std::vector<RsExpr *> parts)
      : RsExpr(Kind::Concat), parts(std::move(parts)) {}

  static bool classof(const RsExpr *expr) { return expr->kind == Kind::Concat; }

  std::string print() const override {
    std::string result;
    for (auto *part : parts) {
      result += part->print();
      result += ' ';
    }
    return result;
  }

  void ForEachChild(llvm::function_ref<void(RsExpr *&)> fn) override {
    for (auto *&part : parts) {
      fn(part);
    }
  }

  std::vector<RsExpr *> parts;
};

struct Delim : RsExpr {
  Delim(char open, char close, RsExpr *inner)
      : RsExpr(Kind::Delim), open(open), close(close), inner(inner) {}

  static bool classof(const RsExpr *expr) { return expr->kind == Kind::Delim; }

  std::string print() const override {
    std::string result;
    result += open;
    result += ' ';
    result += inner->print();
    result += close;
    result += ' ';
    return result;
  }

  void ForEachChild(llvm::function_ref<void(RsExpr *&)> fn) override {
    fn(inner);
  }

  RsExpr *Pointer() override { return inner->Pointer(); }

  char open;
  char close;
  RsExpr *inner;
};

struct Unary : RsExpr {
  enum class Op : uint8_t {
    Deref,
    Not,
    Neg,
  };

  Unary(Op op, RsExpr *operand)
      : RsExpr(Kind::Unary), op(op), operand(operand) {}

  static bool classof(const RsExpr *expr) { return expr->kind == Kind::Unary; }

  std::string print() const override {
    switch (op) {
    case Op::Deref:
      return "*" + operand->print();
    case Op::Not:
      return "!" + operand->print();
    case Op::Neg:
      return "-" + operand->print();
    }
    std::unreachable();
  }

  void ForEachChild(llvm::function_ref<void(RsExpr *&)> fn) override {
    fn(operand);
  }

  RsExpr *Pointer() override { return op == Op::Deref ? operand : nullptr; }

  Op op;
  RsExpr *operand;
};

struct Assign : RsExpr {
  Assign(RsExpr *left, RsExpr *right)
      : RsExpr(Kind::Assign), left(left), right(right) {}

  static bool classof(const RsExpr *expr) { return expr->kind == Kind::Assign; }

  std::string print() const override {
    return left->print() + " = " + right->print() + ' ';
  }

  void ForEachChild(llvm::function_ref<void(RsExpr *&)> fn) override {
    fn(left);
    fn(right);
  }

  RsExpr *left;
  RsExpr *right;
};

struct CompoundAssign : RsExpr {
  CompoundAssign(RsExpr *left, std::string op, RsExpr *right)
      : RsExpr(Kind::CompoundAssign), left(left), op(std::move(op)),
        right(right) {}

  static bool classof(const RsExpr *expr) {
    return expr->kind == Kind::CompoundAssign;
  }

  std::string print() const override {
    return left->print() + ' ' + op + ' ' + right->print() + ' ';
  }

  void ForEachChild(llvm::function_ref<void(RsExpr *&)> fn) override {
    fn(left);
    fn(right);
  }

  RsExpr *left;
  std::string op;
  RsExpr *right;
};

struct MethodCall : RsExpr {
  MethodCall(RsExpr *receiver, std::string method, std::vector<RsExpr *> args)
      : RsExpr(Kind::MethodCall), receiver(receiver), method(std::move(method)),
        args(std::move(args)) {}

  static bool classof(const RsExpr *expr) {
    return expr->kind == Kind::MethodCall;
  }

  std::string print() const override {
    std::string result = receiver->print();
    result += '.';
    result += method;
    result += '(';
    for (size_t i = 0; i < args.size(); ++i) {
      if (i > 0) {
        result += ',';
      }
      result += args[i]->print();
    }
    result += ") ";
    return result;
  }

  void ForEachChild(llvm::function_ref<void(RsExpr *&)> fn) override {
    fn(receiver);
    for (auto *&arg : args) {
      fn(arg);
    }
  }

  RsExpr *receiver;
  std::string method;
  std::vector<RsExpr *> args;
};

inline bool SameRendered(const RsExpr *lhs, const RsExpr *rhs) {
  return lhs->print() == rhs->print();
}

class RsArena {
public:
  template <typename T, typename... Args> T *New(Args &&...args) {
    pool_.push_back(std::make_unique<T>(std::forward<Args>(args)...));
    return static_cast<T *>(pool_.back().get());
  }

private:
  std::vector<std::unique_ptr<RsExpr>> pool_;
};

} // namespace cpp2rust
