// translation-fail: refcount
// no-compile: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.virtual]/4: a virtual function marked final cannot be
// overridden further, but it is still a virtual function: a call through a
// base pointer reaches it, also for objects of classes derived from the class
// that declared it final.
#include <cassert>

static int base_calls, final_calls;

struct DeclaresVirtual {
  virtual void f() { ++base_calls; }
};

struct OverridesAsFinal : DeclaresVirtual {
  void f() final { ++final_calls; }
};

struct DerivesFromFinalOverrider : OverridesAsFinal {};

struct FinalClass final : DeclaresVirtual {
  void f() override { ++final_calls; }
};

int main() {
  DerivesFromFinalOverrider d;
  DeclaresVirtual *p = &d;
  p->f();
  assert(base_calls == 0 && final_calls == 1);
  OverridesAsFinal *q = &d;
  q->f();
  assert(base_calls == 0 && final_calls == 2);

  FinalClass c;
  p = &c;
  p->f();
  assert(base_calls == 0 && final_calls == 3);
  c.f();
  assert(base_calls == 0 && final_calls == 4);
  return 0;
}
