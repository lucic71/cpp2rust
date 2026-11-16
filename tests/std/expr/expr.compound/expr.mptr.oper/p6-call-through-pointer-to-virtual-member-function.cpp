// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.mptr.oper]/6 and [expr.call]/2: the result of ->* on a pointer
// to member function is called with the function call operator; if the
// function is virtual, its final overrider in the dynamic type of the object
// expression is called.
#include <cassert>

static int base_calls, derived_calls, non_virtual_calls;

struct DeclaresVirtual {
  virtual void f() { ++base_calls; }
  void g() { ++non_virtual_calls; }
};

struct Overrides : DeclaresVirtual {
  void f() override { ++derived_calls; }
};

int main() {
  void (DeclaresVirtual::*member)() = &DeclaresVirtual::f;

  Overrides d;
  DeclaresVirtual *base = &d;
  (base->*member)();
  assert(base_calls == 0 && derived_calls == 1);

  DeclaresVirtual onlyBase;
  (onlyBase.*member)();
  assert(base_calls == 1 && derived_calls == 1);

  member = &DeclaresVirtual::g;
  (base->*member)();
  assert(non_virtual_calls == 1);
  assert(base_calls == 1 && derived_calls == 1);
  return 0;
}
