// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.virtual]/3: a virtual member function does not have to be
// visible to be overridden. HidesWithOverload::f(int) hides DeclaresVirtual::f()
// and is not virtual; OverridesHiddenVirtual::f() overrides DeclaresVirtual::f()
// even though that function is not visible in OverridesHiddenVirtual.
#include <cassert>

static int base_calls, overload_calls, overrider_calls;

struct DeclaresVirtual {
  virtual void f() { ++base_calls; }
};

struct HidesWithOverload : DeclaresVirtual {
  void f(int) { ++overload_calls; }
};

struct OverridesHiddenVirtual : HidesWithOverload {
  void f() { ++overrider_calls; }
};

int main() {
  OverridesHiddenVirtual d;
  DeclaresVirtual *base = &d;
  base->f();
  assert(base_calls == 0 && overload_calls == 0 && overrider_calls == 1);

  HidesWithOverload *hides = &d;
  hides->f(1);
  assert(base_calls == 0 && overload_calls == 1 && overrider_calls == 1);
  hides->DeclaresVirtual::f();
  assert(base_calls == 1 && overload_calls == 1 && overrider_calls == 1);

  HidesWithOverload onlyHides;
  base = &onlyHides;
  base->f();
  assert(base_calls == 2 && overload_calls == 1 && overrider_calls == 1);
  return 0;
}
