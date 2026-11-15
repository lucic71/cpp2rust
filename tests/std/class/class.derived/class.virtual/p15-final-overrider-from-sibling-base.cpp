// translation-fail: refcount
// no-compile: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.virtual]/15: InheritsVirtually does not declare f, so in
// JoinsSiblings the final overrider of the shared DeclaresVirtual::f is
// OverridesVirtually::f. A call through an InheritsVirtually pointer, whose
// class is unrelated to OverridesVirtually, calls OverridesVirtually::f.
#include <cassert>

static int base_calls, sibling_calls;

struct DeclaresVirtual {
  virtual void f() { ++base_calls; }
};

struct InheritsVirtually : virtual DeclaresVirtual {};

struct OverridesVirtually : virtual DeclaresVirtual {
  void f() override { ++sibling_calls; }
};

struct JoinsSiblings : InheritsVirtually, OverridesVirtually {};

int main() {
  JoinsSiblings d;
  InheritsVirtually *inherits = &d;
  inherits->f();
  assert(base_calls == 0 && sibling_calls == 1);

  DeclaresVirtual *base = inherits;
  base->f();
  assert(base_calls == 0 && sibling_calls == 2);

  InheritsVirtually onlyInherits;
  inherits = &onlyInherits;
  inherits->f();
  assert(base_calls == 1 && sibling_calls == 2);
  return 0;
}
