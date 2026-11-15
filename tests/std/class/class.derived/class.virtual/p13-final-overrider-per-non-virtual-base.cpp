// translation-fail: refcount
// no-compile: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.virtual]/13: HasTwoBaseSubobjects has two separate
// DeclaresVirtual subobjects and hence two occurrences of DeclaresVirtual::f.
// FirstOverrider::f is the final overrider of the one inside FirstOverrider,
// and SecondOverrider::f of the one inside SecondOverrider.
#include <cassert>

static int base_calls, first_calls, second_calls;

struct DeclaresVirtual {
  virtual void f() { ++base_calls; }
};

struct FirstOverrider : DeclaresVirtual {
  void f() override { ++first_calls; }
};

struct SecondOverrider : DeclaresVirtual {
  void f() override { ++second_calls; }
};

struct HasTwoBaseSubobjects : FirstOverrider, SecondOverrider {};

int main() {
  HasTwoBaseSubobjects d;
  FirstOverrider *first = &d;
  DeclaresVirtual *base = first;
  base->f();
  assert(base_calls == 0 && first_calls == 1 && second_calls == 0);

  SecondOverrider *second = &d;
  base = second;
  base->f();
  assert(base_calls == 0 && first_calls == 1 && second_calls == 1);

  first->f();
  assert(base_calls == 0 && first_calls == 2 && second_calls == 1);
  second->f();
  assert(base_calls == 0 && first_calls == 2 && second_calls == 2);
  return 0;
}
