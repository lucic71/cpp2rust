// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "rs_expr.h"

#include <llvm/Support/Casting.h>
#include <llvm/Support/raw_ostream.h>

namespace cpp2rust {

RsExpr *RsExpr::IgnoreParens() {
  auto *node = this;
  while (auto *delim = llvm::dyn_cast<Delim>(node)) {
    if (delim->open != '(' || delim->close != ')') {
      break;
    }
    node = delim->inner;
  }
  return node;
}

PtrWith *RsExpr::TakeWithFrom(RsExpr *&slot) {
  if (auto *with = llvm::dyn_cast<PtrWith>(slot)) {
    slot = llvm::cast<Closure>(with->closure)->body;
    return with;
  }
  return slot->TakeWith();
}

const char *KindName(RsExpr::Kind kind) {
  switch (kind) {
  case RsExpr::Kind::Verbatim:
    return "Verbatim";
  case RsExpr::Kind::Concat:
    return "Concat";
  case RsExpr::Kind::Delim:
    return "Delim";
  case RsExpr::Kind::Unary:
    return "Unary";
  case RsExpr::Kind::Cast:
    return "Cast";
  case RsExpr::Kind::Call:
    return "Call";
  case RsExpr::Kind::Closure:
    return "Closure";
  case RsExpr::Kind::Assign:
    return "Assign";
  case RsExpr::Kind::CompoundAssign:
    return "CompoundAssign";
  case RsExpr::Kind::Fn:
    return "Fn";
  case RsExpr::Kind::Trait:
    return "Trait";
  case RsExpr::Kind::Impl:
    return "Impl";
  case RsExpr::Kind::Field:
    return "Field";
  case RsExpr::Kind::Index:
    return "Index";
  case RsExpr::Kind::FieldPtr:
    return "FieldPtr";
  case RsExpr::Kind::BorrowRead:
    return "BorrowRead";
  case RsExpr::Kind::BorrowWrite:
    return "BorrowWrite";
  case RsExpr::Kind::PtrRead:
    return "PtrRead";
  case RsExpr::Kind::PtrWrite:
    return "PtrWrite";
  case RsExpr::Kind::PtrWith:
    return "PtrWith";
  }
  return "Unknown";
}

namespace {

void DumpHeader(const RsExpr *node, llvm::raw_ostream &os, unsigned depth,
                const std::string &details = {}) {
  os.indent(depth * 2) << KindName(node->kind);
  if (!details.empty()) {
    os << ' ' << details;
  }
  os << '\n';
}

void DumpChildren(RsExpr *node, llvm::raw_ostream &os, unsigned depth) {
  node->ForEachChild([&](RsExpr *&child) { child->dump(os, depth + 1); });
}

} // namespace

bool RsExpr::ContainsBorrow() {
  bool found = false;
  ForEachChild(
      [&](RsExpr *&child) { found = found || child->ContainsBorrow(); });
  return found;
}

void RsExpr::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth);
  DumpChildren(this, os, depth);
}

void Verbatim::dump(llvm::raw_ostream &os, unsigned depth) {
  std::string line = text;
  for (auto &ch : line) {
    if (ch == '\n') {
      ch = ' ';
    }
  }
  if (line.size() > 60) {
    line.resize(60);
    line += "...";
  }
  DumpHeader(this, os, depth, '"' + line + '"');
}

void Concat::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth);
  DumpChildren(this, os, depth);
}

void Delim::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, std::string(1, open) + std::string(1, close));
  DumpChildren(this, os, depth);
}

void Unary::dump(llvm::raw_ostream &os, unsigned depth) {
  const char *name = "";
  switch (op) {
  case Op::Deref:
    name = "deref";
    break;
  case Op::Not:
    name = "not";
    break;
  case Op::Neg:
    name = "neg";
    break;
  }
  DumpHeader(this, os, depth, name);
  DumpChildren(this, os, depth);
}

void Cast::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth);
  DumpChildren(this, os, depth);
}

void Call::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, is_mut ? "mut" : "");
  DumpChildren(this, os, depth);
}

void Closure::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, '|' + param + '|');
  DumpChildren(this, os, depth);
}

void Fn::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, name);
  DumpChildren(this, os, depth);
}

void Trait::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, name);
  DumpChildren(this, os, depth);
}

void Impl::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, trait_name);
  DumpChildren(this, os, depth);
}

void Field::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, '.' + member);
  DumpChildren(this, os, depth);
}

void Index::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth);
  DumpChildren(this, os, depth);
}

void FieldPtr::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth,
             type_name + "::" + field + " @" + std::to_string(offset) +
                 (container ? " container" : ""));
  DumpChildren(this, os, depth);
}

void BorrowRead::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth);
  DumpChildren(this, os, depth);
}

void BorrowWrite::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth);
  DumpChildren(this, os, depth);
}

void PtrRead::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth);
  DumpChildren(this, os, depth);
}

void PtrWrite::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth);
  DumpChildren(this, os, depth);
}

void PtrWith::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, is_mut ? "mut" : "");
  DumpChildren(this, os, depth);
}

void Assign::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth);
  DumpChildren(this, os, depth);
}

void CompoundAssign::dump(llvm::raw_ostream &os, unsigned depth) {
  DumpHeader(this, os, depth, op);
  DumpChildren(this, os, depth);
}

} // namespace cpp2rust
