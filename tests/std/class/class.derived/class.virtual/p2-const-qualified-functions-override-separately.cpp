// translation-fail: refcount
// no-compile: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.virtual]/2: a function overrides a virtual function only if
// their declarations correspond ([basic.scope.scope]/4), which requires the
// same cv-qualifiers on the function. The const and non-const overloads of f
// are overridden separately, and a const f in a derived class does not
// override a non-const f of the base: it hides it.
#include <cassert>

static int base_non_const_calls, base_const_calls, derived_const_calls;
static int non_const_only_calls, added_const_calls;

struct HasConstAndNonConst {
  virtual void f() { ++base_non_const_calls; }
  virtual void f() const { ++base_const_calls; }
};

struct OverridesConstOnly : HasConstAndNonConst {
  void f() const override { ++derived_const_calls; }
  using HasConstAndNonConst::f;
};

struct HasNonConstOnly {
  virtual void f() { ++non_const_only_calls; }
};

struct AddsConst : HasNonConstOnly {
  void f() const { ++added_const_calls; }
};

int main() {
  OverridesConstOnly d;
  HasConstAndNonConst &nonConstView = d;
  nonConstView.f();
  assert(base_non_const_calls == 1);
  assert(base_const_calls == 0 && derived_const_calls == 0);

  const HasConstAndNonConst &constView = d;
  constView.f();
  assert(derived_const_calls == 1);
  assert(base_non_const_calls == 1 && base_const_calls == 0);

  AddsConst a;
  HasNonConstOnly *base = &a;
  base->f();
  assert(non_const_only_calls == 1 && added_const_calls == 0);

  a.f();
  assert(non_const_only_calls == 1 && added_const_calls == 1);
  return 0;
}
