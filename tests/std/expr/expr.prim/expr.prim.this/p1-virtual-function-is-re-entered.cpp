// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.prim.this]/1: the keyword this names a pointer to the object for
// which the member function is invoked.
#include <cassert>

struct DeclaresVirtuals {
  int value = 0;
  virtual void run() = 0;
  virtual void increment() = 0;
};

static DeclaresVirtuals *other_pointer_to_object;

static void incrementThroughOtherPointer() {
  other_pointer_to_object->increment();
}

struct ReEntered : DeclaresVirtuals {
  void run() override {
    value = 10;
    incrementThroughOtherPointer();
    value += 100;
  }
  void increment() override { ++value; }
};

int main() {
  ReEntered d;
  other_pointer_to_object = &d;
  DeclaresVirtuals *base = &d;

  base->run();
  assert(d.value == 111);

  base->increment();
  assert(d.value == 112);
  return 0;
}
