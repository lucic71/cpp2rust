// panic: refcount
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [conv.ptr]/3: a pointer to a derived class D can be converted to a
// pointer to a base class B of D; the result is a pointer to the base class
// subobject of the derived class object.
#include <cassert>

struct DeclaresVirtual {
  virtual int id() = 0;
};

struct HoldsId : DeclaresVirtual {
  int held = 0;
  int id() override { return held; }
};

struct HasPolymorphicMembers {
  int before = 0;
  HoldsId first;
  HoldsId second;
};

int main() {
  HasPolymorphicMembers owner;
  owner.first.held = 4;
  owner.second.held = 5;

  DeclaresVirtual *base = &owner.first;
  assert(base->id() == 4);
  base = &owner.second;
  assert(base->id() == 5);

  owner.second.held = 6;
  assert(base->id() == 6);
  return 0;
}
