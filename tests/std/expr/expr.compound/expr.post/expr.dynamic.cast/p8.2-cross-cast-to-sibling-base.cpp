// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.dynamic.cast]/8.2: otherwise, if v points to a public base class
// subobject of the most derived object, and the most derived object has an
// unambiguous public base class of type C, the result points to that C
// subobject.
#include <cassert>

struct FirstBase {
  int first = 1;
  virtual void f() {}
};

struct SecondBase {
  int second = 2;
  virtual void g() {}
};

struct MostDerived : FirstBase, SecondBase {};

int main() {
  MostDerived d;
  FirstBase *first = &d;

  SecondBase *second = dynamic_cast<SecondBase *>(first);
  SecondBase *expected = &d;
  assert(second == expected);
  assert(second->second == 2);

  second->second = 5;
  assert(d.second == 5);
  assert(d.first == 1);

  FirstBase *back = dynamic_cast<FirstBase *>(second);
  assert(back == first);
  return 0;
}
