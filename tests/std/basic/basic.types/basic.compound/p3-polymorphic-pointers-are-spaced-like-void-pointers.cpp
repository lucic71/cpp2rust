// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.compound]/3: the value representation of pointer types is
// implementation-defined but they have to be compatible with void*.
#include <cassert>

struct DeclaresVirtual {
  virtual void f() {}
};

struct HoldsPolymorphicPointer {
  DeclaresVirtual *pointer;
  int after;
};

struct HoldsVoidPointer {
  void *pointer;
  int after;
};

static long bytesBetween(void *from, void *to) {
  return static_cast<char *>(to) - static_cast<char *>(from);
}

int main() {
  DeclaresVirtual *polymorphicPointers[3] = {};
  void *voidPointers[3] = {};
  long polymorphicStride =
      bytesBetween(&polymorphicPointers[0], &polymorphicPointers[1]);
  long voidStride = bytesBetween(&voidPointers[0], &voidPointers[1]);
  assert(polymorphicStride == voidStride);
  assert(bytesBetween(&polymorphicPointers[0], &polymorphicPointers[2]) ==
         2 * voidStride);

  HoldsPolymorphicPointer holdsPolymorphic = {};
  HoldsVoidPointer holdsVoid = {};
  long polymorphicOffset =
      bytesBetween(&holdsPolymorphic, &holdsPolymorphic.after);
  long voidOffset = bytesBetween(&holdsVoid, &holdsVoid.after);
  assert(polymorphicOffset == voidOffset);
  return 0;
}
