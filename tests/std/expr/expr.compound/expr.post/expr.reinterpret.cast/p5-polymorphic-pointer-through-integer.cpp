// translation-fail: refcount
// no-compile: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.reinterpret.cast]/5: a pointer converted to an integer of
// sufficient size and back to the same pointer type has its original value.
#include <cassert>
#include <cstdint>

static int base_calls, derived_calls;

struct DeclaresVirtual {
  virtual void f() { ++base_calls; }
};

struct Overrides : DeclaresVirtual {
  void f() override { ++derived_calls; }
};

int main() {
  Overrides d;
  DeclaresVirtual *original = &d;

  std::uintptr_t asInteger = reinterpret_cast<std::uintptr_t>(original);
  DeclaresVirtual *restored = reinterpret_cast<DeclaresVirtual *>(asInteger);
  assert(restored == original);

  restored->f();
  assert(base_calls == 0 && derived_calls == 1);
  return 0;
}
