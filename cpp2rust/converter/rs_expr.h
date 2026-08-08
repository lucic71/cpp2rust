#pragma once

// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <clang/AST/Type.h>
#include <llvm/ADT/STLFunctionalExtras.h>

#include <cstdint>
#include <format>
#include <memory>
#include <optional>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

namespace clang {
class Expr;
}

namespace llvm {
class raw_ostream;
}

namespace cpp2rust {

struct PtrWith;

struct RsExpr {
  enum class Kind : uint8_t {
    Verbatim,
    Concat,
    Delim,
    Unary,
    Cast,
    Call,
    Closure,
    Assign,
    CompoundAssign,
    Fn,
    Trait,
    Impl,
    Field,
    Index,
    FieldPtr,
    PtrView,
    BitField,
    BorrowRead,
    BorrowWrite,
    PtrRead,
    PtrWrite,
    PtrWith,
  };

  explicit RsExpr(Kind kind) : kind(kind) {}
  virtual ~RsExpr() = default;

  virtual std::string print() const = 0;

  virtual void dump(llvm::raw_ostream &os, unsigned depth = 0);

  virtual bool ContainsBorrow();

  virtual void ForEachChild(llvm::function_ref<void(RsExpr *&)>) {}

  RsExpr *IgnoreParens();

  template <typename T> T *Find() {
    if (auto *node = llvm::dyn_cast<T>(this)) {
      return node;
    }
    T *found = nullptr;
    ForEachChild([&](RsExpr *&child) {
      if (found == nullptr) {
        found = child->Find<T>();
      }
    });
    return found;
  }

  virtual RsExpr *Pointer() { return nullptr; }

  // Removes the innermost `with` from the receiver chain, splicing its closure
  // body in its place.
  virtual PtrWith *TakeWith() { return nullptr; }

  static PtrWith *TakeWithFrom(RsExpr *&slot);

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

  void dump(llvm::raw_ostream &os, unsigned depth = 0) override;

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

  void dump(llvm::raw_ostream &os, unsigned depth = 0) override;
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

  RsExpr *TakePtr(RsExpr *replacement) override {
    if (auto *ptr = inner->Pointer()) {
      inner = replacement;
      return ptr;
    }
    return inner->TakePtr(replacement);
  }

  RsExpr *Pointer() override {
    return open == '(' ? inner->Pointer() : nullptr;
  }

  PtrWith *TakeWith() override { return TakeWithFrom(inner); }

  char open;
  char close;
  RsExpr *inner;

  void dump(llvm::raw_ostream &os, unsigned depth = 0) override;
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
      return "(*" + operand->print() + ")";
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

  PtrWith *TakeWith() override { return TakeWithFrom(operand); }

  Op op;
  RsExpr *operand;

  void dump(llvm::raw_ostream &os, unsigned depth = 0) override;
};

struct Cast : RsExpr {
  Cast(RsExpr *expr, RsExpr *type, clang::QualType pointee = {});

  static bool classof(const RsExpr *e) { return e->kind == Kind::Cast; }

  std::string print() const override {
    return '(' + expr->print() + " as " + type->print() + ')';
  }

  void ForEachChild(llvm::function_ref<void(RsExpr *&)> fn) override {
    fn(expr);
    fn(type);
  }

  RsExpr *TakePtr(RsExpr *replacement) override {
    if (auto *ptr = expr->Pointer()) {
      expr = replacement;
      return ptr;
    }
    return expr->TakePtr(replacement);
  }

  PtrWith *TakeWith() override { return TakeWithFrom(expr); }

  RsExpr *expr;
  RsExpr *type;

  void dump(llvm::raw_ostream &os, unsigned depth = 0) override;
};

struct Call : RsExpr {
  Call(RsExpr *callee, std::vector<RsExpr *> args, bool is_mut = false)
      : RsExpr(Kind::Call), callee(callee), args(std::move(args)),
        is_mut(is_mut) {}

  static bool classof(const RsExpr *e) { return e->kind == Kind::Call; }

  std::string print() const override {
    std::string result = callee->print();
    result += '(';
    for (size_t i = 0; i < args.size(); ++i) {
      if (i > 0) {
        result += ',';
      }
      result += args[i]->print();
    }
    return result + ") ";
  }

  void ForEachChild(llvm::function_ref<void(RsExpr *&)> fn) override {
    fn(callee);
    for (auto *&arg : args) {
      fn(arg);
    }
  }

  RsExpr *TakePtr(RsExpr *replacement) override {
    if (auto *ptr = callee->Pointer()) {
      callee = replacement;
      return ptr;
    }
    return callee->TakePtr(replacement);
  }

  PtrWith *TakeWith() override { return TakeWithFrom(callee); }

  RsExpr *callee;
  std::vector<RsExpr *> args;
  bool is_mut;

  void dump(llvm::raw_ostream &os, unsigned depth = 0) override;
};

struct Closure : RsExpr {
  Closure(std::string param, RsExpr *param_type, RsExpr *body)
      : RsExpr(Kind::Closure), param(std::move(param)), param_type(param_type),
        body(body) {}

  static bool classof(const RsExpr *e) { return e->kind == Kind::Closure; }

  std::string print() const override {
    std::string result = '|' + param;
    if (param_type) {
      result += ": " + param_type->print();
    }
    result += "| ";
    return result + body->print();
  }

  void ForEachChild(llvm::function_ref<void(RsExpr *&)> fn) override {
    if (param_type) {
      fn(param_type);
    }
    fn(body);
  }

  std::string param;
  RsExpr *param_type;
  RsExpr *body;

  void dump(llvm::raw_ostream &os, unsigned depth = 0) override;
};

inline std::string PrintWords(const std::vector<RsExpr *> &nodes) {
  std::string result;
  for (auto *node : nodes) {
    auto text = node->print();
    if (!text.empty()) {
      result += text + ' ';
    }
  }
  return result;
}

struct Fn : RsExpr {
  enum class Receiver : uint8_t { None, Ref, RefMut };

  Fn(std::vector<RsExpr *> qualifiers, std::string name, Receiver receiver,
     std::vector<RsExpr *> params, RsExpr *return_type,
     std::optional<std::vector<RsExpr *>> body)
      : RsExpr(Kind::Fn), qualifiers(std::move(qualifiers)),
        name(std::move(name)), receiver(receiver), params(std::move(params)),
        return_type(return_type), body(std::move(body)) {}

  static bool classof(const RsExpr *e) { return e->kind == Kind::Fn; }

  std::string print() const override {
    std::string result = PrintWords(qualifiers) + "fn " + name + '(';
    switch (receiver) {
    case Receiver::None:
      break;
    case Receiver::Ref:
      result += "&self, ";
      break;
    case Receiver::RefMut:
      result += "&mut self, ";
      break;
    }
    for (size_t i = 0; i < params.size(); ++i) {
      result += params[i]->print();
      if (i + 1 != params.size()) {
        result += ", ";
      }
    }
    result += ") ";
    if (return_type) {
      result += return_type->print() + ' ';
    }
    if (!body) {
      return result + "; ";
    }
    result += '{';
    for (auto *stmt : *body) {
      result += stmt->print();
    }
    return result + "} ";
  }

  void ForEachChild(llvm::function_ref<void(RsExpr *&)> fn) override {
    for (auto *&qualifier : qualifiers) {
      fn(qualifier);
    }
    for (auto *&param : params) {
      fn(param);
    }
    if (return_type) {
      fn(return_type);
    }
    if (body) {
      for (auto *&stmt : *body) {
        fn(stmt);
      }
    }
  }

  std::vector<RsExpr *> qualifiers;
  std::string name;
  Receiver receiver;
  std::vector<RsExpr *> params;
  RsExpr *return_type;
  std::optional<std::vector<RsExpr *>> body;

  void dump(llvm::raw_ostream &os, unsigned depth = 0) override;
};

struct Trait : RsExpr {
  Trait(std::vector<RsExpr *> qualifiers, std::string name,
        std::vector<RsExpr *> items)
      : RsExpr(Kind::Trait), qualifiers(std::move(qualifiers)),
        name(std::move(name)), items(std::move(items)) {}

  static bool classof(const RsExpr *e) { return e->kind == Kind::Trait; }

  std::string print() const override {
    std::string result = PrintWords(qualifiers) + "trait " + name + " {";
    for (auto *item : items) {
      result += item->print();
    }
    return result + "} ";
  }

  void ForEachChild(llvm::function_ref<void(RsExpr *&)> fn) override {
    for (auto *&qualifier : qualifiers) {
      fn(qualifier);
    }
    for (auto *&item : items) {
      fn(item);
    }
  }

  std::vector<RsExpr *> qualifiers;
  std::string name;
  std::vector<RsExpr *> items;

  void dump(llvm::raw_ostream &os, unsigned depth = 0) override;
};

struct Impl : RsExpr {
  Impl(std::vector<RsExpr *> qualifiers, std::string trait_name,
       RsExpr *self_type, std::vector<RsExpr *> items)
      : RsExpr(Kind::Impl), qualifiers(std::move(qualifiers)),
        trait_name(std::move(trait_name)), self_type(self_type),
        items(std::move(items)) {}

  static bool classof(const RsExpr *e) { return e->kind == Kind::Impl; }

  std::string print() const override {
    std::string result = PrintWords(qualifiers) + "impl ";
    if (!trait_name.empty()) {
      result += trait_name + " for ";
    }
    result += self_type->print() + " {";
    for (auto *item : items) {
      result += item->print();
    }
    return result + "} ";
  }

  void ForEachChild(llvm::function_ref<void(RsExpr *&)> fn) override {
    for (auto *&qualifier : qualifiers) {
      fn(qualifier);
    }
    fn(self_type);
    for (auto *&item : items) {
      fn(item);
    }
  }

  std::vector<RsExpr *> qualifiers;
  std::string trait_name;
  RsExpr *self_type;
  std::vector<RsExpr *> items;

  void dump(llvm::raw_ostream &os, unsigned depth = 0) override;
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

  PtrWith *TakeWith() override { return TakeWithFrom(object); }

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

  void dump(llvm::raw_ostream &os, unsigned depth = 0) override;
};

struct BitField : Accessor {
  BitField(RsExpr *object, std::string member, std::string type_name,
           std::string promoted_name, bool base_has_side_effects)
      : Accessor(Kind::BitField, object), member(std::move(member)),
        type_name(std::move(type_name)),
        promoted_name(std::move(promoted_name)),
        base_has_side_effects(base_has_side_effects) {}

  static bool classof(const RsExpr *expr) {
    return expr->kind == Kind::BitField;
  }

  std::string print() const override {
    return object->print() + '.' + member + "() ";
  }

  std::string Setter() const { return "set_" + member; }

  std::string member;
  std::string type_name;
  std::string promoted_name;
  bool base_has_side_effects;

  void dump(llvm::raw_ostream &os, unsigned depth = 0) override;
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

  void dump(llvm::raw_ostream &os, unsigned depth = 0) override;
};

struct FieldPtr : Accessor {
  FieldPtr(RsExpr *object, size_t offset, std::string type_name,
           std::string field, clang::QualType field_type, bool container)
      : Accessor(Kind::FieldPtr, object), offset(offset),
        type_name(std::move(type_name)), field(std::move(field)),
        field_type(field_type), container(container) {}

  static bool classof(const RsExpr *expr) {
    return expr->kind == Kind::FieldPtr;
  }

  std::string print() const override {
    auto get = element
                   ? std::format("|__v: &{}| &__v.{}[..]", type_name, field)
                   : std::format("|__v: &{}| ::std::slice::from_ref(&__v.{})",
                                 type_name, field);
    auto get_mut =
        element
            ? std::format("|__v: &mut {}| &mut __v.{}[..]", type_name, field)
            : std::format("|__v: &mut {}| ::std::slice::from_mut(&mut __v.{})",
                          type_name, field);
    return object->print() +
           std::format(".field_ptr({}, {}, {})", offset, get, get_mut);
  }

  size_t offset;
  std::string type_name;
  std::string field;
  clang::QualType field_type;
  bool container;
  bool element = false;

  void dump(llvm::raw_ostream &os, unsigned depth = 0) override;
};

struct PtrView : Accessor {
  PtrView(RsExpr *object, clang::QualType view_type)
      : Accessor(Kind::PtrView, object), view_type(view_type) {}

  static bool classof(const RsExpr *expr) {
    return expr->kind == Kind::PtrView;
  }

  std::string print() const override {
    return object->print() +
           (element ? ".to_strong().as_pointer()" : ".clone()");
  }

  clang::QualType view_type;
  bool element = false;

  void dump(llvm::raw_ostream &os, unsigned depth = 0) override;
};

struct BorrowRead : Accessor {
  explicit BorrowRead(RsExpr *object) : Accessor(Kind::BorrowRead, object) {}

  bool ContainsBorrow() override { return true; }

  static bool classof(const RsExpr *expr) {
    return expr->kind == Kind::BorrowRead;
  }

  std::string print() const override {
    return "(*" + object->print() + ".borrow()) ";
  }

  void dump(llvm::raw_ostream &os, unsigned depth = 0) override;
};

struct BorrowWrite : Accessor {
  explicit BorrowWrite(RsExpr *object) : Accessor(Kind::BorrowWrite, object) {}

  bool ContainsBorrow() override { return true; }

  static bool classof(const RsExpr *expr) {
    return expr->kind == Kind::BorrowWrite;
  }

  std::string print() const override {
    return "(*" + object->print() + ".borrow_mut()) ";
  }

  void dump(llvm::raw_ostream &os, unsigned depth = 0) override;
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

  void dump(llvm::raw_ostream &os, unsigned depth = 0) override;
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

  void dump(llvm::raw_ostream &os, unsigned depth = 0) override;
};

struct PtrWith : Accessor {
  PtrWith(RsExpr *object, bool is_mut, RsExpr *closure)
      : Accessor(Kind::PtrWith, object), is_mut(is_mut), closure(closure) {}

  static bool classof(const RsExpr *expr) {
    return expr->kind == Kind::PtrWith;
  }

  std::string print() const override {
    return object->print() + (is_mut ? ".with_mut(" : ".with(") +
           closure->print() + ") ";
  }

  void ForEachChild(llvm::function_ref<void(RsExpr *&)> fn) override {
    fn(object);
    fn(closure);
  }

  void dump(llvm::raw_ostream &os, unsigned depth = 0) override;

  bool is_mut;
  RsExpr *closure;
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

  void dump(llvm::raw_ostream &os, unsigned depth = 0) override;
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

  void dump(llvm::raw_ostream &os, unsigned depth = 0) override;
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

const char *KindName(RsExpr::Kind kind);

} // namespace cpp2rust
