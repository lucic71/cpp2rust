// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/ir_src.h"

#include <cassert>
#include <cstdlib>

#include "logging.h"

namespace cpp2rust::IrSrc {

namespace {

using Kind = Node::Kind;

constexpr std::pair<Kind, const char *> kKindNames[] = {
    {Kind::kNone, "none"},
    {Kind::kParam, "param"},
    {Kind::kValue, "value"},
    {Kind::kBuiltin, "builtin"},
    {Kind::kRecord, "record"},
    {Kind::kEnum, "enum"},
    {Kind::kTypedef, "typedef"},
    {Kind::kPointer, "ptr"},
    {Kind::kLRef, "lref"},
    {Kind::kRRef, "rref"},
    {Kind::kArray, "array"},
    {Kind::kIncompleteArray, "iarray"},
    {Kind::kFunctionType, "fntype"},
    {Kind::kPack, "pack"},
    {Kind::kOpaque, "opaque"},
    {Kind::kFunction, "function"},
    {Kind::kDecl, "decl"},
    {Kind::kMacro, "macro"},
    {Kind::kUnary, "unary"},
    {Kind::kArrow, "arrow"},
    {Kind::kPackParams, "pack_params"},
};

const char *kindName(Kind kind) {
  for (const auto &[k, name] : kKindNames) {
    if (k == kind) {
      return name;
    }
  }
  assert(0 && "unknown IR node kind");
  return "";
}

Kind kindFromName(llvm::StringRef name) {
  for (const auto &[k, n] : kKindNames) {
    if (name == n) {
      return k;
    }
  }
  llvm::errs() << "ERROR: unknown IR node kind '" << name << "'\n";
  std::exit(EXIT_FAILURE);
}

Node ParseNodeJSON(const llvm::json::Value &value) {
  const auto *obj = value.getAsObject();
  assert(obj && "IR node must be an object");
  Node node;
  node.kind = kindFromName(*obj->getString("k"));
  if (auto name = obj->getString("n")) {
    node.name = name->str();
  }
  if (auto param = obj->getInteger("i")) {
    node.param = *param;
  }
  node.is_const = obj->getBoolean("c").value_or(false);
  node.is_volatile = obj->getBoolean("v").value_or(false);
  node.variadic = obj->getBoolean("va").value_or(false);
  if (auto ref = obj->getString("r")) {
    node.ref = ref->str();
  }
  if (const auto *children = obj->getArray("a")) {
    for (const auto &child : *children) {
      node.children.push_back(ParseNodeJSON(child));
    }
  }
  return node;
}

ExprRule ParseExprRuleJSON(const llvm::json::Object &obj) {
  ExprRule rule;
  rule.ir = ParseNodeJSON(*obj.get("ir"));
  if (const auto *init_type = obj.getObject("init_type")) {
    rule.init_type = InitTypeLocation{
        (unsigned)*init_type->getInteger("depth"),
        (unsigned)*init_type->getInteger("index"),
    };
  }
  return rule;
}

TypeRule ParseTypeRuleJSON(const llvm::json::Object &obj) {
  TypeRule rule;
  rule.ir = ParseNodeJSON(*obj.get("ir"));
  return rule;
}

} // namespace

bool Node::operator==(const Node &other) const {
  return kind == other.kind && name == other.name && param == other.param &&
         is_const == other.is_const && is_volatile == other.is_volatile &&
         variadic == other.variadic && ref == other.ref &&
         children == other.children;
}

unsigned Node::specificity() const {
  unsigned n = kind == Kind::kParam ? 0 : 1;
  for (const auto &child : children) {
    n += child.specificity();
  }
  return n;
}

void Node::forEachParam(const std::function<void(unsigned)> &fn) const {
  if (kind == Kind::kParam) {
    fn(param);
  }
  for (const auto &child : children) {
    child.forEachParam(fn);
  }
}

std::string Node::str() const {
  std::string out;
  if (is_const) {
    out += "const ";
  }
  if (is_volatile) {
    out += "volatile ";
  }
  if (kind == Kind::kParam) {
    return out + "T" + std::to_string(param);
  }
  out += kindName(kind);
  if (!name.empty()) {
    out += " " + name;
  }
  if (!children.empty()) {
    out += "(";
    for (size_t i = 0; i < children.size(); ++i) {
      out += (i ? ", " : "") + children[i].str();
    }
    out += ")";
  }
  return out;
}

llvm::json::Value ToJSON(const Node &node) {
  llvm::json::Object obj{{"k", kindName(node.kind)}};
  if (!node.name.empty()) {
    obj["n"] = node.name;
  }
  if (node.kind == Kind::kParam) {
    obj["i"] = node.param;
  }
  if (node.is_const) {
    obj["c"] = true;
  }
  if (node.is_volatile) {
    obj["v"] = true;
  }
  if (node.variadic) {
    obj["va"] = true;
  }
  if (!node.ref.empty()) {
    obj["r"] = node.ref;
  }
  if (!node.children.empty()) {
    llvm::json::Array children;
    for (const auto &child : node.children) {
      children.push_back(ToJSON(child));
    }
    obj["a"] = std::move(children);
  }
  return obj;
}

void ExprRule::dump() const {
  log() << "Matching: " << ir.str() << '\n';
  if (init_type.valid()) {
    log() << "  init type: depth " << init_type.depth << ", index "
          << init_type.index << '\n';
  }
}

void TypeRule::dump() const { log() << "name: " << ir.str() << '\n'; }

Rules Load(const std::filesystem::path &dir) {
  Rules rules;
  Ir::LoadJSON(rules, dir / "ir_src.json", true, ParseExprRuleJSON,
               ParseTypeRuleJSON);
  return rules;
}

} // namespace cpp2rust::IrSrc
