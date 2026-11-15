// translation-fail: refcount
// no-compile: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.abstract]/5: an abstract class can be derived from a class that
// is not abstract, and a pure virtual function can override a virtual function
// which is not pure. HasImplementation::f has a body; MakesPureAgain redeclares
// f as pure, which makes it abstract; ImplementsAgain overrides f once more.
#include <cassert>

static int base_calls, final_calls;

struct HasImplementation {
  virtual void f() { ++base_calls; }
};

struct MakesPureAgain : HasImplementation {
  void f() override = 0;
};

struct ImplementsAgain : MakesPureAgain {
  void f() override { ++final_calls; }
};

int main() {
  HasImplementation onlyBase;
  onlyBase.f();
  assert(base_calls == 1 && final_calls == 0);

  ImplementsAgain d;

  HasImplementation *asBase = &d;
  asBase->f();
  assert(base_calls == 1 && final_calls == 1);

  MakesPureAgain *asAbstract = &d;
  asAbstract->f();
  assert(base_calls == 1 && final_calls == 2);

  d.HasImplementation::f();
  assert(base_calls == 2 && final_calls == 2);
  return 0;
}
