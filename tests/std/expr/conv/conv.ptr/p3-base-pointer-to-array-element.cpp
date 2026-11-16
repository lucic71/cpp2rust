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

int main() {
  HoldsId elements[3];
  elements[0].held = 1;
  elements[1].held = 2;
  elements[2].held = 3;

  DeclaresVirtual *base = &elements[1];
  assert(base->id() == 2);
  base = &elements[2];
  assert(base->id() == 3);
  base = &elements[0];
  assert(base->id() == 1);

  HoldsId *heap = new HoldsId[2];
  heap[0].held = 8;
  heap[1].held = 9;
  base = &heap[1];
  assert(base->id() == 9);
  base = heap;
  assert(base->id() == 8);
  delete[] heap;
  return 0;
}
