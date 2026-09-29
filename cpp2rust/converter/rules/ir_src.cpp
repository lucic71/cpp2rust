// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/ir_src.h"

#include <algorithm>
#include <cassert>
#include <cstdlib>

#include "logging.h"

namespace cpp2rust::IrSrc {

namespace {

using Kind = Node::Kind;
using llvm::cast;
using llvm::dyn_cast;

constexpr std::pair<Kind, const char *> kKindNames[] = {
    {Kind::kParam, "param"},         {Kind::kValue, "value"},
    {Kind::kBuiltin, "builtin"},     {Kind::kRecord, "record"},
    {Kind::kEnum, "enum"},           {Kind::kTypedef, "typedef"},
    {Kind::kConst, "const"},         {Kind::kVolatile, "volatile"},
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

NodePtr ParseNodeJSON(const llvm::json::Value &value) {
  const auto *obj = value.getAsObject();
  assert(obj && "IR node must be an object");
  auto kind = kindFromName(*obj->getString("kind"));
  auto name = obj->getString("name").value_or("").str();
  NodePtr node;
  switch (kind) {
  case Kind::kParam:
    node = std::make_unique<ParamNode>(*obj->getInteger("param"));
    break;
  case Kind::kConst:
  case Kind::kVolatile:
    node = std::make_unique<QualNode>(kind, nullptr);
    break;
  case Kind::kPointer:
  case Kind::kLRef:
  case Kind::kRRef:
    node = std::make_unique<PointerNode>(kind, nullptr);
    break;
  case Kind::kArray:
    node = std::make_unique<ArrayNode>(nullptr);
    break;
  case Kind::kRecord:
    node = std::make_unique<RecordNode>(std::move(name));
    break;
  case Kind::kDecl:
    node = std::make_unique<DeclNode>(std::move(name));
    break;
  case Kind::kFunction:
  case Kind::kFunctionType: {
    auto fn = std::make_unique<FunctionNode>(kind, std::move(name));
    fn->variadic = obj->getBoolean("variadic").value_or(false);
    fn->is_const = obj->getBoolean("is_const").value_or(false);
    fn->is_volatile = obj->getBoolean("is_volatile").value_or(false);
    fn->ref = obj->getString("ref").value_or("").str();
    node = std::move(fn);
    break;
  }
  case Kind::kUnary:
    node = std::make_unique<UnaryNode>(std::move(name), nullptr);
    break;
  case Kind::kArrow:
    node = std::make_unique<ArrowNode>(nullptr, nullptr);
    break;
  default:
    node = std::make_unique<Node>(kind, std::move(name));
    break;
  }
  forEachField(*node, *node, [&](const char *key, auto &field, auto &) {
    if constexpr (kIsNodeList<decltype(field)>) {
      if (const auto *list = obj->getArray(key)) {
        for (const auto &child : *list) {
          field.push_back(ParseNodeJSON(child));
        }
      }
    } else if (const auto *child = obj->get(key)) {
      field = ParseNodeJSON(*child);
    }
    return true;
  });
  return node;
}

unsigned NumParams(const Node &ir) {
  unsigned n = 0;
  ir.forEachParam([&](unsigned param) { n = std::max(n, param); });
  assert(n <= Ir::kMaxGenerics && "template placeholder exceeds kMaxGenerics");
  return n;
}

ExprRule ParseExprRuleJSON(const llvm::json::Object &obj) {
  ExprRule rule;
  rule.ir = ParseNodeJSON(*obj.get("ir"));
  rule.num_params = NumParams(*rule.ir);
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
  rule.num_params = NumParams(*rule.ir);
  return rule;
}

} // namespace

bool Node::operator==(const Node &other) const {
  return shallowEquals(other) &&
         zipChildren(*this, other, [](const NodePtr &a, const NodePtr &b) {
           return *a == *b;
         });
}

bool Node::shallowEquals(const Node &other) const {
  if (kind != other.kind || name != other.name) {
    return false;
  }
  if (const auto *param = dyn_cast<ParamNode>(this)) {
    return param->param == cast<ParamNode>(other).param;
  }
  if (const auto *fn = dyn_cast<FunctionNode>(this)) {
    const auto &other_fn = cast<FunctionNode>(other);
    return fn->variadic == other_fn.variadic &&
           fn->is_const == other_fn.is_const &&
           fn->is_volatile == other_fn.is_volatile && fn->ref == other_fn.ref;
  }
  return true;
}

std::string Node::indexKey() const {
  switch (kind) {
  case Kind::kFunction:
  case Kind::kDecl:
  case Kind::kRecord:
  case Kind::kEnum:
  case Kind::kTypedef:
  case Kind::kBuiltin:
    return name;
  case Kind::kMacro:
    return "macro:" + name;
  case Kind::kUnary:
    return "unary" + name + ":" + cast<UnaryNode>(this)->operand->indexKey();
  case Kind::kArrow:
    return cast<ArrowNode>(this)->member->indexKey();
  case Kind::kConst:
  case Kind::kVolatile:
    return cast<QualNode>(this)->operand->indexKey();
  case Kind::kPointer:
    return "*" + cast<PointerNode>(this)->pointee->indexKey();
  case Kind::kLRef:
    return "&" + cast<PointerNode>(this)->pointee->indexKey();
  case Kind::kRRef:
    return "&&" + cast<PointerNode>(this)->pointee->indexKey();
  case Kind::kArray:
    return "[]" + cast<ArrayNode>(this)->element->indexKey();
  default:
    return "";
  }
}

unsigned Node::specificity() const {
  unsigned n = kind == Kind::kParam ? 0 : 1;
  zipChildren(*this, *this, [&](const NodePtr &child, const NodePtr &) {
    n += child->specificity();
    return true;
  });
  return n;
}

bool Node::hasParam(unsigned n) const {
  if (const auto *param = dyn_cast<ParamNode>(this)) {
    return param->param == n;
  }
  return !zipChildren(*this, *this, [&](const NodePtr &child, const NodePtr &) {
    return !child->hasParam(n);
  });
}

void Node::forEachParam(const std::function<void(unsigned)> &fn) const {
  if (const auto *param = dyn_cast<ParamNode>(this)) {
    fn(param->param);
  }
  zipChildren(*this, *this, [&](const NodePtr &child, const NodePtr &) {
    child->forEachParam(fn);
    return true;
  });
}

std::string Node::str() const {
  if (const auto *param = dyn_cast<ParamNode>(this)) {
    return "T" + std::to_string(param->param);
  }
  if (const auto *qual = dyn_cast<QualNode>(this)) {
    return std::string(kindName(kind)) + " " + qual->operand->str();
  }
  std::string out;
  if (const auto *fn = dyn_cast<FunctionNode>(this)) {
    if (fn->is_const) {
      out += "const ";
    }
    if (fn->is_volatile) {
      out += "volatile ";
    }
  }
  out += kindName(kind);
  if (!name.empty()) {
    out += " " + name;
  }
  std::vector<std::string> fields;
  forEachField(*this, *this,
               [&](const char *key, const auto &field, const auto &) {
                 if constexpr (kIsNodeList<decltype(field)>) {
                   if (!field.empty()) {
                     std::string list;
                     for (const auto &child : field) {
                       list += (list.empty() ? "" : ", ") + child->str();
                     }
                     fields.push_back(std::string(key) + ": [" + list + "]");
                   }
                 } else if (field) {
                   fields.push_back(std::string(key) + ": " + field->str());
                 }
                 return true;
               });
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
  if (const auto *param = dyn_cast<ParamNode>(&node)) {
    obj["param"] = param->param;
  }
  if (const auto *fn = dyn_cast<FunctionNode>(&node)) {
    if (fn->is_const) {
      obj["is_const"] = true;
    }
    if (fn->is_volatile) {
      obj["is_volatile"] = true;
    }
    if (fn->variadic) {
      obj["variadic"] = true;
    }
    if (!fn->ref.empty()) {
      obj["ref"] = fn->ref;
    }
  }
  forEachField(node, node,
               [&](const char *key, const auto &field, const auto &) {
                 if constexpr (kIsNodeList<decltype(field)>) {
                   if (!field.empty()) {
                     llvm::json::Array list;
                     for (const auto &child : field) {
                       list.push_back(ToJSON(*child));
                     }
                     obj[key] = std::move(list);
                   }
                 } else if (field) {
                   obj[key] = ToJSON(*field);
                 }
                 return true;
               });
  return obj;
}

void ExprRule::dump() const {
  log() << "Matching: " << ir->str() << '\n';
  if (init_type.valid()) {
    log() << "  init type: depth " << init_type.depth << ", index "
          << init_type.index << '\n';
  }
}

void TypeRule::dump() const { log() << "name: " << ir->str() << '\n'; }

Rules Load(const std::filesystem::path &dir) {
  Rules rules;
  Ir::LoadJSON(rules, dir / "ir_src.json", true, ParseExprRuleJSON,
               ParseTypeRuleJSON);
  return rules;
}

} // namespace cpp2rust::IrSrc
