// translation-fail: refcount
// no-compile: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.types.general]/3: for two objects of a trivially copyable type,
// copying the underlying bytes of one into the other gives it the same value.
#include <cassert>
#include <cstring>

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

  DeclaresVirtual *copied = nullptr;
  std::memcpy(&copied, &original, sizeof(original));
  assert(copied == original);

  copied->f();
  assert(base_calls == 0 && derived_calls == 1);
  return 0;
}
