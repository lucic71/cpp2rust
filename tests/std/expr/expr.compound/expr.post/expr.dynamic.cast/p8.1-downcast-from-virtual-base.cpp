// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.dynamic.cast]/8.1: if, in the most derived object pointed to by
// v, v points to a public base class subobject of a C object, and only one
// object of type C is derived from that subobject, the result points to that
// C object.
#include <cassert>

struct VirtualBase {
  int shared = 1;
  virtual void f() {}
};

struct FirstPath : virtual VirtualBase {
  int first = 2;
};

struct SecondPath : virtual VirtualBase {
  int second = 3;
};

struct JoinsPaths : FirstPath, SecondPath {
  int own = 4;
};

int main() {
  JoinsPaths d;
  VirtualBase *base = &d;

  JoinsPaths *toJoin = dynamic_cast<JoinsPaths *>(base);
  assert(toJoin == &d);
  assert(toJoin->own == 4);

  SecondPath *toSecond = dynamic_cast<SecondPath *>(base);
  SecondPath *expected = &d;
  assert(toSecond == expected);
  assert(toSecond->second == 3);
  return 0;
}
