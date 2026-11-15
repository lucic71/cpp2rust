// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.abstract]/6: member functions can be called from a constructor
// or destructor of an abstract class. A virtual function that is not pure
// resolves to the abstract class's own version there, not to the overrider in
// the class derived from it.
#include <cassert>

static int abstract_hook_calls, derived_hook_calls, helper_calls;

struct CallsHookInConstructor {
  CallsHookInConstructor() {
    helper();
    hook();
  }
  ~CallsHookInConstructor() {
    helper();
    hook();
  }
  void helper() { ++helper_calls; }
  virtual void hook() { ++abstract_hook_calls; }
  virtual void pure() = 0;
};

struct OverridesHook : CallsHookInConstructor {
  void hook() override { ++derived_hook_calls; }
  void pure() override {}
};

int main() {
  {
    OverridesHook d;
    assert(helper_calls == 1);
    assert(abstract_hook_calls == 1 && derived_hook_calls == 0);
    CallsHookInConstructor *p = &d;
    p->hook();
    assert(abstract_hook_calls == 1 && derived_hook_calls == 1);
  }
  assert(helper_calls == 2);
  assert(abstract_hook_calls == 2 && derived_hook_calls == 1);
  return 0;
}
