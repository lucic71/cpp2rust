// translation-fail: refcount
// no-compile: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.static.cast]/14 and [basic.compound]/6: a pointer to void can
// hold any object pointer, and converting it back to the original pointer type
// leaves the pointer value unchanged.
#include <cassert>

static int base_calls, derived_calls;

struct DeclaresVirtual {
  virtual void f() { ++base_calls; }
};

struct Overrides : DeclaresVirtual {
  void f() override { ++derived_calls; }
};

static void callThroughVoidPointer(void *context) {
  static_cast<DeclaresVirtual *>(context)->f();
}

int main() {
  Overrides d;
  DeclaresVirtual *original = &d;

  void *erased = original;
  DeclaresVirtual *restored = static_cast<DeclaresVirtual *>(erased);
  assert(restored == original);

  restored->f();
  assert(base_calls == 0 && derived_calls == 1);

  callThroughVoidPointer(original);
  assert(base_calls == 0 && derived_calls == 2);

  DeclaresVirtual plain;
  callThroughVoidPointer(&plain);
  assert(base_calls == 1 && derived_calls == 2);
  return 0;
}
