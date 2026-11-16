// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.static.cast]/12: a pointer to a base class B can be converted to
// a pointer to a class D derived from B. If it points to a B that is a base
// class subobject of a D object, the result points to the enclosing D object.
#include <cassert>

static int derived_only_calls;

struct Base {
  int base = 1;
  virtual void f() {}
};

struct Derived : Base {
  int derived = 2;
  void derivedOnly() { ++derived_only_calls; }
};

int main() {
  Derived d;
  Base *base = &d;

  Derived *derived = static_cast<Derived *>(base);
  assert(derived == &d);
  assert(derived->derived == 2);

  derived->derivedOnly();
  assert(derived_only_calls == 1);

  derived->derived = 5;
  assert(d.derived == 5);
  assert(d.base == 1);
  return 0;
}
