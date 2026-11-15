// translation-fail: refcount
// no-compile: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.virtual]/2: a using-declaration names a base function but does
// not override it. In NamesBaseFunctionWithUsing, d.f() calls
// OverridesVirtually::f, the final overrider; the qualified call
// d.NamesBaseFunctionWithUsing::f() calls DeclaresVirtual::f, the function the
// using-declaration names.
#include <cassert>

static int base_calls, overrider_calls;

struct DeclaresVirtual {
  virtual void f() { ++base_calls; }
};

struct OverridesVirtually : virtual DeclaresVirtual {
  void f() override { ++overrider_calls; }
};

struct NamesBaseFunctionWithUsing : OverridesVirtually, virtual DeclaresVirtual {
  using DeclaresVirtual::f;
};

int main() {
  NamesBaseFunctionWithUsing d;
  d.f();
  assert(base_calls == 0 && overrider_calls == 1);
  d.NamesBaseFunctionWithUsing::f();
  assert(base_calls == 1 && overrider_calls == 1);

  DeclaresVirtual *base = &d;
  base->f();
  assert(base_calls == 1 && overrider_calls == 2);
  return 0;
}
