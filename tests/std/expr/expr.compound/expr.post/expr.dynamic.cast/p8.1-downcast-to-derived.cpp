// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.dynamic.cast]/8.1: if, in the most derived object pointed to by
// v, v points to a public base class subobject of a C object, and only one
// object of type C is derived from that subobject, the result points to that
// C object.
#include <cassert>

struct Base {
  int base = 1;
  virtual void f() {}
};

struct Intermediate : Base {
  int intermediate = 2;
};

struct MostDerived : Intermediate {
  int mostDerived = 3;
};

int main() {
  MostDerived d;
  Base *base = &d;

  MostDerived *toMostDerived = dynamic_cast<MostDerived *>(base);
  assert(toMostDerived == &d);
  assert(toMostDerived->mostDerived == 3);

  Intermediate *toIntermediate = dynamic_cast<Intermediate *>(base);
  assert(toIntermediate == &d);
  assert(toIntermediate->intermediate == 2);
  return 0;
}
