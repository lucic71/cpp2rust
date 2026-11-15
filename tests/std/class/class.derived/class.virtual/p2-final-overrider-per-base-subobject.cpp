// translation-fail: refcount
// no-compile: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.virtual]/2: HasBothPaths has two DeclaresVirtual subobjects.
// DeclaresVirtual::f is the final overrider for the one inside
// InheritsWithoutOverriding, and Overrides::f for the one inside Overrides, so
// the function called depends on which subobject the call goes through.
#include <cassert>

static int base_calls, overrider_calls;

struct DeclaresVirtual {
  virtual void f() { ++base_calls; }
};

struct InheritsWithoutOverriding : DeclaresVirtual {};

struct Overrides : DeclaresVirtual {
  void f() override { ++overrider_calls; }
};

struct HasBothPaths : InheritsWithoutOverriding, Overrides {};

int main() {
  HasBothPaths d;
  InheritsWithoutOverriding *notOverridden = &d;
  Overrides *overridden = &d;
  notOverridden->f();
  assert(base_calls == 1 && overrider_calls == 0);
  overridden->f();
  assert(base_calls == 1 && overrider_calls == 1);

  DeclaresVirtual *baseOfNotOverridden = notOverridden;
  DeclaresVirtual *baseOfOverridden = overridden;
  baseOfNotOverridden->f();
  assert(base_calls == 2 && overrider_calls == 1);
  baseOfOverridden->f();
  assert(base_calls == 2 && overrider_calls == 2);
  return 0;
}
