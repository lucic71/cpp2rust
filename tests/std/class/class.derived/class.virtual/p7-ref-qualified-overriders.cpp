// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.virtual]/7: the ref-qualifier of an overriding function is the
// same as that of the overridden function. The & and && overloads of f are
// overridden separately; the value category of the object expression selects
// the overload and the dynamic type selects the overrider.
#include <cassert>

static int base_lvalue_calls, base_rvalue_calls;
static int derived_lvalue_calls, derived_rvalue_calls;

struct HasRefQualifiedVirtuals {
  virtual void f() & { ++base_lvalue_calls; }
  virtual void f() && { ++base_rvalue_calls; }
};

struct OverridesBoth : HasRefQualifiedVirtuals {
  void f() & override { ++derived_lvalue_calls; }
  void f() && override { ++derived_rvalue_calls; }
};

struct OverridesRvalueOnly : HasRefQualifiedVirtuals {
  void f() && override { ++derived_rvalue_calls; }
  using HasRefQualifiedVirtuals::f;
};

int main() {
  OverridesBoth both;
  HasRefQualifiedVirtuals &r = both;
  r.f();
  assert(derived_lvalue_calls == 1 && derived_rvalue_calls == 0);
  static_cast<HasRefQualifiedVirtuals &&>(r).f();
  assert(derived_lvalue_calls == 1 && derived_rvalue_calls == 1);
  assert(base_lvalue_calls == 0 && base_rvalue_calls == 0);

  OverridesRvalueOnly rvalueOnly;
  HasRefQualifiedVirtuals &s = rvalueOnly;
  s.f();
  assert(base_lvalue_calls == 1 && derived_lvalue_calls == 1);
  static_cast<HasRefQualifiedVirtuals &&>(s).f();
  assert(base_rvalue_calls == 0 && derived_rvalue_calls == 2);
  return 0;
}
