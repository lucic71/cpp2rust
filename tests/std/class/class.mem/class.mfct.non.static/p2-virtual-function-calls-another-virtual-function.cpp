// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.mfct.non.static]/2: inside a member function, a name that
// resolves to a non-static member of the class or of a base class is
// transformed into a class member access expression using (*this).
#include <cassert>

struct DeclaresVirtuals {
  int steps = 0;
  virtual void first() = 0;
  virtual void second() = 0;
};

struct FirstCallsSecond : DeclaresVirtuals {
  void first() override {
    ++steps;
    second();
  }
  void second() override { steps += 10; }
};

struct OverridesSecond : FirstCallsSecond {
  void second() override { steps += 100; }
};

int main() {
  FirstCallsSecond d;
  DeclaresVirtuals *base = &d;
  base->first();
  assert(d.steps == 11);

  OverridesSecond e;
  base = &e;
  base->first();
  assert(e.steps == 101);
  assert(d.steps == 11);
  return 0;
}
