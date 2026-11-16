// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.static.cast]/12: a pointer to a base class B can be converted to
// a pointer to a class D derived from B. If it points to a B that is a base
// class subobject of a D object, the result points to the enclosing D object.
#include <cassert>

struct FirstBase {
  int first = 1;
};

struct SecondBase {
  int second = 2;
};

struct HasTwoBases : FirstBase, SecondBase {
  int own = 3;
};

int main() {
  HasTwoBases d;
  SecondBase *second = &d;

  HasTwoBases *derived = static_cast<HasTwoBases *>(second);
  assert(derived == &d);
  assert(derived->first == 1);
  assert(derived->second == 2);
  assert(derived->own == 3);

  derived->first = 10;
  assert(d.first == 10);
  assert(second->second == 2);
  return 0;
}
