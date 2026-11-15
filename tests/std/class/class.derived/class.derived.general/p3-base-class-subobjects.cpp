// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.derived.general]/3: the base-specifier-list specifies the base
// class subobjects contained in an object of the derived class. A RedeclaresC
// object has a RedeclaresB subobject, which has a DeclaresABC subobject. The
// members b and c declared at each level are distinct objects: RedeclaresB::b
// hides DeclaresABC::b, and RedeclaresC::c hides DeclaresABC::c.
#include <cassert>

struct DeclaresABC {
  int a, b, c;
};

struct RedeclaresB : DeclaresABC {
  int b;
};

struct RedeclaresC : RedeclaresB {
  int c;
};

int main() {
  RedeclaresC d;
  d.DeclaresABC::a = 1;
  d.DeclaresABC::b = 2;
  d.DeclaresABC::c = 3;
  d.RedeclaresB::b = 20;
  d.RedeclaresC::c = 30;

  assert(d.a == 1);
  assert(d.b == 20);
  assert(d.c == 30);
  assert(d.DeclaresABC::b == 2);
  assert(d.DeclaresABC::c == 3);

  RedeclaresB &asRedeclaresB = d;
  assert(asRedeclaresB.a == 1);
  assert(asRedeclaresB.b == 20);
  assert(asRedeclaresB.c == 3);

  DeclaresABC &asDeclaresABC = d;
  assert(asDeclaresABC.a == 1);
  assert(asDeclaresABC.b == 2);
  assert(asDeclaresABC.c == 3);

  asDeclaresABC.b = 4;
  asRedeclaresB.c = 5;
  assert(d.b == 20);
  assert(d.c == 30);
  assert(d.DeclaresABC::b == 4);
  assert(d.DeclaresABC::c == 5);
  return 0;
}
