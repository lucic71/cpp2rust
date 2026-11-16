// translation-fail: refcount
// no-compile: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.static.cast]/14: converting a pointer to void and back to the
// same pointer type leaves the pointer value unchanged.
#include <cassert>

static int second_base_calls, derived_calls;

struct FirstBase {
  int first = 1;
  virtual void f() {}
};

struct SecondBase {
  int second = 2;
  virtual void g() { ++second_base_calls; }
};

struct OverridesSecond : FirstBase, SecondBase {
  void g() override { ++derived_calls; }
};

int main() {
  OverridesSecond d;
  SecondBase *original = &d;

  void *erased = original;
  SecondBase *restored = static_cast<SecondBase *>(erased);
  assert(restored == original);
  assert(restored->second == 2);

  restored->g();
  assert(second_base_calls == 0 && derived_calls == 1);

  restored->second = 5;
  assert(d.second == 5);
  assert(d.first == 1);
  return 0;
}
