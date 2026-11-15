// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.cdtor]/4: when a virtual function is called directly or
// indirectly from a constructor or destructor for the object under
// construction or destruction, the function called is the final overrider in
// the constructor's or destructor's class, not one in a more-derived class.
#include <cassert>

static int base_hook_calls, first_hook_calls, second_hook_calls;

struct CallsHook {
  CallsHook() { hook(); }
  ~CallsHook() { callHookIndirectly(); }
  void callHookIndirectly() { hook(); }
  virtual void hook() { ++base_hook_calls; }
};

struct OverridesHookAndCallsIt : CallsHook {
  OverridesHookAndCallsIt() { callHookIndirectly(); }
  ~OverridesHookAndCallsIt() { hook(); }
  void hook() override { ++first_hook_calls; }
};

struct OverridesHookAgain : OverridesHookAndCallsIt {
  void hook() override { ++second_hook_calls; }
};

int main() {
  {
    OverridesHookAgain d;
    assert(base_hook_calls == 1);
    assert(first_hook_calls == 1);
    assert(second_hook_calls == 0);

    d.callHookIndirectly();
    assert(second_hook_calls == 1);
    CallsHook *p = &d;
    p->hook();
    assert(second_hook_calls == 2);
    assert(base_hook_calls == 1 && first_hook_calls == 1);
  }
  assert(base_hook_calls == 2);
  assert(first_hook_calls == 2);
  assert(second_hook_calls == 2);
  return 0;
}
