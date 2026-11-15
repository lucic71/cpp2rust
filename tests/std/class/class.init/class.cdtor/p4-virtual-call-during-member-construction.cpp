// translation-fail: refcount
// no-compile: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.cdtor]/4 and [class.base.init]/16: the rule for virtual calls
// from a constructor covers the construction of the class's non-static data
// members. A mem-initializer and a default member initializer call hook()
// after the bases are initialized; both get the constructor's class's version.
#include <cassert>

static int base_hook_calls, first_hook_calls, second_hook_calls;

struct DeclaresHook {
  virtual int hook() { return ++base_hook_calls; }
};

struct CallsHookInMemberInitializers : DeclaresHook {
  int fromMemInitializer;
  int fromDefaultMemberInitializer = hook();
  CallsHookInMemberInitializers() : fromMemInitializer(hook()) {}
  int hook() override { return ++first_hook_calls; }
};

struct OverridesHookAgain : CallsHookInMemberInitializers {
  int hook() override { return ++second_hook_calls; }
};

int main() {
  OverridesHookAgain d;
  assert(d.fromMemInitializer == 1);
  assert(d.fromDefaultMemberInitializer == 2);
  assert(base_hook_calls == 0);
  assert(first_hook_calls == 2);
  assert(second_hook_calls == 0);

  DeclaresHook *p = &d;
  assert(p->hook() == 1);
  assert(second_hook_calls == 1);
  return 0;
}
