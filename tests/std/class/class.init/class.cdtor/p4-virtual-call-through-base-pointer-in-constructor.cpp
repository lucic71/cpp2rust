// translation-fail: refcount
// no-compile: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.cdtor]/4: while the constructor of CallsVirtualsInConstructor
// runs for a base subobject of MostDerived, a virtual call uses the final
// overrider in CallsVirtualsInConstructor. overriddenBySibling() runs the
// VirtualBase version, not the sibling's; overriddenHere() runs this class's
// version, not MostDerived's, also through a pointer to the virtual base.
#include <cassert>

static int base_sibling_calls, base_here_calls;
static int sibling_calls, here_calls;
static int most_derived_sibling_calls, most_derived_here_calls;

struct VirtualBase {
  virtual void overriddenBySibling() { ++base_sibling_calls; }
  virtual void overriddenHere() { ++base_here_calls; }
};

struct SiblingOverrider : virtual VirtualBase {
  void overriddenBySibling() override { ++sibling_calls; }
};

struct CallsVirtualsInConstructor : virtual VirtualBase {
  void overriddenHere() override { ++here_calls; }
  CallsVirtualsInConstructor(VirtualBase *base);
};

struct MostDerived : SiblingOverrider, CallsVirtualsInConstructor {
  void overriddenBySibling() override { ++most_derived_sibling_calls; }
  void overriddenHere() override { ++most_derived_here_calls; }
  MostDerived()
      : CallsVirtualsInConstructor(static_cast<SiblingOverrider *>(this)) {}
};

CallsVirtualsInConstructor::CallsVirtualsInConstructor(VirtualBase *base) {
  overriddenBySibling();
  assert(base_sibling_calls == 1);
  assert(sibling_calls == 0 && most_derived_sibling_calls == 0);

  overriddenHere();
  assert(here_calls == 1);
  assert(base_here_calls == 0 && most_derived_here_calls == 0);

  base->overriddenHere();
  assert(here_calls == 2);
  assert(base_here_calls == 0 && most_derived_here_calls == 0);
}

int main() {
  MostDerived d;
  assert(base_sibling_calls == 1 && here_calls == 2);

  VirtualBase *base = &d;
  base->overriddenBySibling();
  assert(most_derived_sibling_calls == 1 && most_derived_here_calls == 0);
  base->overriddenHere();
  assert(most_derived_sibling_calls == 1 && most_derived_here_calls == 1);
  assert(base_sibling_calls == 1 && base_here_calls == 0);
  assert(sibling_calls == 0 && here_calls == 2);
  return 0;
}
