// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.base.init]/7 and /14: a mem-initializer naming a virtual base
// class is ignored during execution of a constructor of any class that is not
// the most derived class. PassesArgumentToVirtualBase forwards its argument
// only when it is the most derived class; inside JoinsBoth that mem-initializer
// is ignored and VirtualBase is default-initialized.
#include <cassert>

static int default_ctors, int_ctors;

struct VirtualBase {
  int value;
  VirtualBase() : value(-1) { ++default_ctors; }
  VirtualBase(int i) : value(i) { ++int_ctors; }
};

struct PassesArgumentToVirtualBase : virtual VirtualBase {
  PassesArgumentToVirtualBase(int i) : VirtualBase(i) {}
};

struct HasNoInitializerForVirtualBase : virtual VirtualBase {
  HasNoInitializerForVirtualBase(int) {}
};

struct JoinsBoth : PassesArgumentToVirtualBase,
                   HasNoInitializerForVirtualBase,
                   virtual VirtualBase {
  JoinsBoth(int i)
      : PassesArgumentToVirtualBase(i), HasNoInitializerForVirtualBase(i) {}
};

int main() {
  VirtualBase base(1);
  assert(default_ctors == 0 && int_ctors == 1);
  assert(base.value == 1);

  PassesArgumentToVirtualBase passes(2);
  assert(default_ctors == 0 && int_ctors == 2);
  assert(passes.value == 2);

  HasNoInitializerForVirtualBase hasNone(3);
  assert(default_ctors == 1 && int_ctors == 2);
  assert(hasNone.value == -1);

  JoinsBoth joined(4);
  assert(default_ctors == 2 && int_ctors == 2);
  assert(joined.value == -1);
  return 0;
}
