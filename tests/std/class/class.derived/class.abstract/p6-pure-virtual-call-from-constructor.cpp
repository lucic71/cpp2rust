// panic-ub
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.abstract]/6: the effect of making a virtual call to a pure
// virtual function, directly or indirectly, for the object being created from
// a constructor of the abstract class is undefined. The constructor reaches
// the pure virtual hook through a non-virtual member.
#include <cassert>

static int derived_hook_calls;

struct CallsPureVirtualInConstructor {
  CallsPureVirtualInConstructor() { callHook(); }
  void callHook() { hook(); }
  virtual void hook() = 0;
};

struct OverridesHook : CallsPureVirtualInConstructor {
  void hook() override { ++derived_hook_calls; }
};

int main() {
  OverridesHook d;
  assert(derived_hook_calls == 0);
  return 0;
}
