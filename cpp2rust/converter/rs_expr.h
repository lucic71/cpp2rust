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
    Field,
    Index,
    BorrowRead,
    BorrowWrite,
    MethodCall,
    PtrRead,
    PtrDeref,
    PtrWrite,
    PtrWith,
  };

  explicit RsExpr(Kind kind) : kind(kind) {}
  virtual ~RsExpr() = default;

  virtual std::string print() const = 0;

  virtual void ForEachChild(llvm::function_ref<void(RsExpr *&)>) {}

  RsExpr *IgnoreParens();

  virtual RsExpr *Pointer() { return nullptr; }

  virtual RsExpr *TakePtr(RsExpr *) { return nullptr; }

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

  RsExpr *TakePtr(RsExpr *replacement) override {
    return inner->TakePtr(replacement);
  }

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

struct Accessor : RsExpr {
  Accessor(Kind kind, RsExpr *object) : RsExpr(kind), object(object) {}

  static bool classof(const RsExpr *expr) {
    return expr->kind >= Kind::Field && expr->kind <= Kind::PtrWith;
  }

  void ForEachChild(llvm::function_ref<void(RsExpr *&)> fn) override {
    fn(object);
  }

  RsExpr *TakePtr(RsExpr *replacement) override {
    if (auto *ptr = object->Pointer()) {
      object = replacement;
      return ptr;
    }
    return object->TakePtr(replacement);
  }

  RsExpr *object;
};

struct Field : Accessor {
  Field(RsExpr *object, std::string member)
      : Accessor(Kind::Field, object), member(std::move(member)) {}

  static bool classof(const RsExpr *expr) { return expr->kind == Kind::Field; }

  std::string print() const override {
    return object->print() + '.' + member + ' ';
  }

  std::string member;
};

struct Index : Accessor {
  Index(RsExpr *object, RsExpr *index)
      : Accessor(Kind::Index, object), index(index) {}

  static bool classof(const RsExpr *expr) { return expr->kind == Kind::Index; }

  std::string print() const override {
    return object->print() + "[(" + index->print() + ") as usize]";
  }

  void ForEachChild(llvm::function_ref<void(RsExpr *&)> fn) override {
    fn(object);
    fn(index);
  }

  RsExpr *index;
};

struct BorrowRead : Accessor {
  explicit BorrowRead(RsExpr *object) : Accessor(Kind::BorrowRead, object) {}

  static bool classof(const RsExpr *expr) {
    return expr->kind == Kind::BorrowRead;
  }

  std::string print() const override {
    return "(*" + object->print() + ".borrow()) ";
  }
};

struct BorrowWrite : Accessor {
  explicit BorrowWrite(RsExpr *object) : Accessor(Kind::BorrowWrite, object) {}

  static bool classof(const RsExpr *expr) {
    return expr->kind == Kind::BorrowWrite;
  }

  std::string print() const override {
    return "(*" + object->print() + ".borrow_mut()) ";
  }
};

struct PtrRead : Accessor {
  explicit PtrRead(RsExpr *object) : Accessor(Kind::PtrRead, object) {}

  static bool classof(const RsExpr *expr) {
    return expr->kind == Kind::PtrRead;
  }

  std::string print() const override {
    return '(' + object->print() + ".read()) ";
  }

  RsExpr *Pointer() override { return object; }
};

struct PtrDeref : Accessor {
  explicit PtrDeref(RsExpr *object) : Accessor(Kind::PtrDeref, object) {}

  static bool classof(const RsExpr *expr) {
    return expr->kind == Kind::PtrDeref;
  }

  std::string print() const override {
    return "(*" + object->print() + ".upgrade().deref()) ";
  }

  RsExpr *Pointer() override { return object; }
};

struct PtrWrite : Accessor {
  PtrWrite(RsExpr *object, RsExpr *value)
      : Accessor(Kind::PtrWrite, object), value(value) {}

  static bool classof(const RsExpr *expr) {
    return expr->kind == Kind::PtrWrite;
  }

  std::string print() const override {
    return object->print() + ".write(" + value->print() + ") ";
  }

  void ForEachChild(llvm::function_ref<void(RsExpr *&)> fn) override {
    fn(object);
    fn(value);
  }

  RsExpr *value;
};

struct PtrWith : Accessor {
  PtrWith(RsExpr *object, bool is_mut, RsExpr *body)
      : Accessor(Kind::PtrWith, object), is_mut(is_mut), body(body) {}

  static bool classof(const RsExpr *expr) {
    return expr->kind == Kind::PtrWith;
  }

  std::string print() const override {
    return object->print() + (is_mut ? ".with_mut(|__v| " : ".with(|__v| ") +
           body->print() + ") ";
  }

  void ForEachChild(llvm::function_ref<void(RsExpr *&)> fn) override {
    fn(object);
    fn(body);
  }

  bool is_mut;
  RsExpr *body;
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

struct MethodCall : Accessor {
  MethodCall(RsExpr *object, std::string method, std::vector<RsExpr *> args)
      : Accessor(Kind::MethodCall, object), method(std::move(method)),
        args(std::move(args)) {}

  static bool classof(const RsExpr *expr) {
    return expr->kind == Kind::MethodCall;
  }

  std::string print() const override {
    std::string result = object->print();
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
    fn(object);
    for (auto *&arg : args) {
      fn(arg);
    }
  }

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
