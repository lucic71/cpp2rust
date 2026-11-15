// translation-fail: refcount
// no-compile: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.virtual]/2: a call to a virtual function runs its final
// overrider in the most derived object. A class that does not override f
// keeps the base version (a virtual function overrides itself), and a base
// object copied from a derived one is a most derived object of the base type.
#include <cassert>

static int base_calls, derived_calls;

struct DeclaresVirtual {
  virtual void f() { ++base_calls; }
};

struct Overrides : DeclaresVirtual {
  void f() override { ++derived_calls; }
};

struct DoesNotOverride : DeclaresVirtual {};

static void callThroughReference(DeclaresVirtual &r) { r.f(); }

int main() {
  Overrides d;
  DeclaresVirtual *p = &d;
  p->f();
  assert(base_calls == 0 && derived_calls == 1);
  callThroughReference(d);
  assert(base_calls == 0 && derived_calls == 2);
  d.f();
  assert(base_calls == 0 && derived_calls == 3);

  DoesNotOverride n;
  p = &n;
  p->f();
  assert(base_calls == 1 && derived_calls == 3);
  callThroughReference(n);
  assert(base_calls == 2 && derived_calls == 3);

  DeclaresVirtual copiedFromDerived = d;
  p = &copiedFromDerived;
  p->f();
  assert(base_calls == 3 && derived_calls == 3);
  callThroughReference(copiedFromDerived);
  assert(base_calls == 4 && derived_calls == 3);
  return 0;
}
