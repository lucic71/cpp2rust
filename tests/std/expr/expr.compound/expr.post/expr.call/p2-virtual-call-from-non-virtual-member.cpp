// translation-fail: refcount
// no-compile: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.call]/2: if the selected function is virtual and is not named by
// a qualified-id, its final overrider in the dynamic type of the object
// expression is called.
#include <cassert>

static int base_calls, derived_calls;

struct DeclaresVirtual {
  virtual void f() { ++base_calls; }
  void callsF() { f(); }
};

struct Overrides : DeclaresVirtual {
  void f() override { ++derived_calls; }
};

int main() {
  DeclaresVirtual onlyBase;
  onlyBase.callsF();
  assert(base_calls == 1 && derived_calls == 0);

  Overrides d;
  d.callsF();
  assert(base_calls == 1 && derived_calls == 1);

  DeclaresVirtual *base = &d;
  base->callsF();
  assert(base_calls == 1 && derived_calls == 2);
  return 0;
}
