// translation-fail: refcount
// no-compile: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.life]/8: if a new object is created at the storage location an
// object occupied after its lifetime has ended, a pointer to the original
// object and the name of the original object automatically refer to the new
// object, provided the original is transparently replaceable by the new one.
#include <cassert>
#include <new>

static int constructions, destructions;

struct DeclaresVirtual {
  virtual int value() { return 0; }
};

struct HoldsValue : DeclaresVirtual {
  int held;
  HoldsValue(int v) : held(v) { ++constructions; }
  ~HoldsValue() { ++destructions; }
  int value() override { return held; }
};

int main() {
  HoldsValue object(1);
  HoldsValue *pointer = &object;
  assert(constructions == 1);
  assert(pointer->value() == 1);

  object.~HoldsValue();
  assert(destructions == 1);

  new (&object) HoldsValue(2);
  assert(constructions == 2);
  assert(object.held == 2);
  assert(pointer->held == 2);
  assert(pointer->value() == 2);

  DeclaresVirtual *base = pointer;
  assert(base->value() == 2);
  return 0;
}
