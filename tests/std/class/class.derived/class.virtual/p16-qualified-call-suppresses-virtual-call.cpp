// translation-fail: refcount
// no-compile: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.virtual]/16: explicit qualification with the scope operator
// suppresses the virtual call mechanism. The call DeclaresVirtual::f() inside
// CallsBaseVersion::f really calls the base function and not the overrider, as
// does p->DeclaresVirtual::f() on a CallsBaseVersion object.
#include <cassert>

static int base_calls, derived_calls;

struct DeclaresVirtual {
  virtual void f() { ++base_calls; }
};

struct CallsBaseVersion : DeclaresVirtual {
  void f() override {
    ++derived_calls;
    DeclaresVirtual::f();
  }
};

int main() {
  CallsBaseVersion d;
  DeclaresVirtual *p = &d;
  p->f();
  assert(derived_calls == 1 && base_calls == 1);
  p->DeclaresVirtual::f();
  assert(derived_calls == 1 && base_calls == 2);
  d.DeclaresVirtual::f();
  assert(derived_calls == 1 && base_calls == 3);
  d.CallsBaseVersion::f();
  assert(derived_calls == 2 && base_calls == 4);
  return 0;
}
