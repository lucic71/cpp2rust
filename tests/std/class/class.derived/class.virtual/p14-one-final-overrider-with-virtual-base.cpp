// translation-fail: refcount
// no-compile: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.virtual]/14: FirstVirtualOverrider::f and
// SecondVirtualOverrider::f both override f of the shared virtual base
// DeclaresVirtual. OverridesBoth::f overrides both and is the single final
// overrider, whichever base the call goes through.
#include <cassert>

static int base_calls, first_calls, second_calls, final_calls;

struct DeclaresVirtual {
  virtual void f() { ++base_calls; }
};

struct FirstVirtualOverrider : virtual DeclaresVirtual {
  void f() override { ++first_calls; }
};

struct SecondVirtualOverrider : virtual DeclaresVirtual {
  void f() override { ++second_calls; }
};

struct OverridesBoth : FirstVirtualOverrider, SecondVirtualOverrider {
  void f() override { ++final_calls; }
};

int main() {
  OverridesBoth d;
  DeclaresVirtual *base = &d;
  FirstVirtualOverrider *first = &d;
  SecondVirtualOverrider *second = &d;
  base->f();
  assert(final_calls == 1);
  first->f();
  assert(final_calls == 2);
  second->f();
  assert(final_calls == 3);
  d.f();
  assert(final_calls == 4);
  assert(base_calls == 0 && first_calls == 0 && second_calls == 0);

  d.FirstVirtualOverrider::f();
  assert(base_calls == 0 && first_calls == 1 && second_calls == 0);
  d.SecondVirtualOverrider::f();
  assert(base_calls == 0 && first_calls == 1 && second_calls == 1);
  d.DeclaresVirtual::f();
  assert(base_calls == 1 && first_calls == 1 && second_calls == 1);
  assert(final_calls == 4);
  return 0;
}
