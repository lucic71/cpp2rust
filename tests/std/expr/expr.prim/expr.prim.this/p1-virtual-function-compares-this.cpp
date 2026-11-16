// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.prim.this]/1: the keyword this names a pointer to the object for
// which the member function is invoked.
#include <cassert>

struct DeclaresVirtual {
  virtual bool isSameObject(DeclaresVirtual *other) = 0;
};

struct ComparesThis : DeclaresVirtual {
  bool isSameObject(DeclaresVirtual *other) override { return this == other; }
};

int main() {
  ComparesThis a;
  ComparesThis b;
  DeclaresVirtual *baseOfA = &a;
  DeclaresVirtual *baseOfB = &b;

  assert(baseOfA->isSameObject(baseOfA));
  assert(!baseOfA->isSameObject(baseOfB));
  assert(baseOfB->isSameObject(&b));
  assert(!baseOfB->isSameObject(&a));
  return 0;
}
