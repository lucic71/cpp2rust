// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.inhctor.init]/1: InheritsThroughTwoLevels inherits
// VirtualBase(int) through InheritsOnce and DeletesDefaultConstructor.
// VirtualBase is a virtual base, so the most derived object initializes it
// with the inherited constructor; initializing the other base subobjects does
// not invoke the deleted default constructor or any other VirtualBase
// constructor.
#include <cassert>

static int int_ctors;

struct VirtualBase {
  int value;
  VirtualBase(int v) : value(v) { ++int_ctors; }
};

struct DeletesDefaultConstructor : virtual VirtualBase {
  using VirtualBase::VirtualBase;
  DeletesDefaultConstructor() = delete;
};

struct InheritsOnce : DeletesDefaultConstructor {
  using DeletesDefaultConstructor::DeletesDefaultConstructor;
};

struct InheritsThroughTwoLevels : InheritsOnce, virtual VirtualBase {
  using InheritsOnce::InheritsOnce;
};

int main() {
  InheritsThroughTwoLevels twoLevels(0);
  assert(int_ctors == 1);
  assert(twoLevels.value == 0);

  InheritsOnce once(4);
  assert(int_ctors == 2);
  assert(once.value == 4);
  return 0;
}
