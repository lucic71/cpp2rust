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
    {Kind::kParam, "param"},         {Kind::kValue, "value"},
    {Kind::kBuiltin, "builtin"},     {Kind::kRecord, "record"},
    {Kind::kEnum, "enum"},           {Kind::kTypedef, "typedef"},
    {Kind::kPointer, "ptr"},         {Kind::kLRef, "lref"},
    {Kind::kRRef, "rref"},           {Kind::kArray, "array"},
    {Kind::kFunctionType, "fntype"}, {Kind::kOpaque, "opaque"},
    {Kind::kFunction, "function"},   {Kind::kDecl, "decl"},
    {Kind::kMacro, "macro"},         {Kind::kUnary, "unary"},
    {Kind::kArrow, "arrow"},         {Kind::kPackParams, "pack_params"},
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
  node.kind = kindFromName(*obj->getString("kind"));
  if (auto name = obj->getString("name")) {
    node.name = name->str();
  }
  if (auto param = obj->getInteger("param")) {
    node.param = *param;
  }
  node.is_const = obj->getBoolean("is_const").value_or(false);
  node.is_volatile = obj->getBoolean("is_volatile").value_or(false);
  node.variadic = obj->getBoolean("variadic").value_or(false);
  if (auto ref = obj->getString("ref")) {
    node.ref = ref->str();
  }
  for (const auto &[key, field] : kNodeFields) {
    if (const auto *child = obj->get(key)) {
      node.*field = std::make_shared<Node>(ParseNodeJSON(*child));
    }
  }
  for (const auto &[key, field] : kNodeLists) {
    if (const auto *list = obj->getArray(key)) {
      for (const auto &child : *list) {
        (node.*field).push_back(ParseNodeJSON(child));
      }
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
         zipChildren(*this, other,
                     [](const Node &a, const Node &b) { return a == b; });
}

unsigned Node::specificity() const {
  unsigned n = kind == Kind::kParam ? 0 : 1;
  zipChildren(*this, *this, [&](const Node &child, const Node &) {
    n += child.specificity();
    return true;
  });
  return n;
}

void Node::forEachParam(const std::function<void(unsigned)> &fn) const {
  if (kind == Kind::kParam) {
    fn(param);
  }
  zipChildren(*this, *this, [&](const Node &child, const Node &) {
    child.forEachParam(fn);
    return true;
  });
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
  std::vector<std::string> fields;
  for (const auto &[key, field] : kNodeFields) {
    if (this->*field) {
      fields.push_back(std::string(key) + ": " + (this->*field)->str());
    }
  }
  for (const auto &[key, field] : kNodeLists) {
    if (!(this->*field).empty()) {
      std::string list;
      for (const auto &child : this->*field) {
        list += (list.empty() ? "" : ", ") + child.str();
      }
      fields.push_back(std::string(key) + ": [" + list + "]");
    }
  }
  if (!fields.empty()) {
    out += "(";
    for (size_t i = 0; i < fields.size(); ++i) {
      out += (i ? ", " : "") + fields[i];
    }
    out += ")";
  }
  return out;
}

llvm::json::Value ToJSON(const Node &node) {
  llvm::json::Object obj{{"kind", kindName(node.kind)}};
  if (!node.name.empty()) {
    obj["name"] = node.name;
  }
  if (node.kind == Kind::kParam) {
    obj["param"] = node.param;
  }
  if (node.is_const) {
    obj["is_const"] = true;
  }
  if (node.is_volatile) {
    obj["is_volatile"] = true;
  }
  if (node.variadic) {
    obj["variadic"] = true;
  }
  if (!node.ref.empty()) {
    obj["ref"] = node.ref;
  }
  for (const auto &[key, field] : kNodeFields) {
    if (node.*field) {
      obj[key] = ToJSON(*(node.*field));
    }
  }
  for (const auto &[key, field] : kNodeLists) {
    if (!(node.*field).empty()) {
      llvm::json::Array list;
      for (const auto &child : node.*field) {
        list.push_back(ToJSON(child));
      }
      obj[key] = std::move(list);
    }
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
