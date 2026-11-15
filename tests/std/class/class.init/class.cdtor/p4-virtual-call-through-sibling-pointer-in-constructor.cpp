// panic-ub: refcount
// nondet-result: unsafe
// translation-fail: refcount
// no-compile: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.cdtor]/4: if a virtual call made during construction uses an
// explicit member access whose object expression refers to the complete
// object or one of its other base subobjects, but not to the object under
// construction or one of its own bases, the behavior is undefined. The
// constructor calls f() through a pointer to its sibling base.

static int base_calls, sibling_calls, most_derived_calls;

struct VirtualBase {
  virtual void f() { ++base_calls; }
};

struct SiblingOverrider : virtual VirtualBase {
  void f() override { ++sibling_calls; }
};

struct CallsThroughSiblingInConstructor : virtual VirtualBase {
  CallsThroughSiblingInConstructor(SiblingOverrider *sibling) { sibling->f(); }
};

struct MostDerived : SiblingOverrider, CallsThroughSiblingInConstructor {
  void f() override { ++most_derived_calls; }
  MostDerived() : CallsThroughSiblingInConstructor(this) {}
};

int main() {
  MostDerived d;
  return base_calls + 2 * sibling_calls + 4 * most_derived_calls;
}
