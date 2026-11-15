// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.virtual]/1: a member function is virtual if it is first declared
// with the keyword virtual or if it overrides a virtual member function of a
// base class. OverridesWithoutKeyword::f is virtual without the keyword, so a
// call through a pointer to that class still reaches the most derived f.
#include <cassert>

static int base_calls, first_calls, second_calls;

struct DeclaresVirtual {
  virtual void f() { ++base_calls; }
};

struct OverridesWithoutKeyword : DeclaresVirtual {
  void f() { ++first_calls; }
};

struct OverridesAgain : OverridesWithoutKeyword {
  void f() { ++second_calls; }
};

int main() {
  OverridesAgain d;
  OverridesWithoutKeyword *first = &d;
  first->f();
  assert(base_calls == 0 && first_calls == 0 && second_calls == 1);

  DeclaresVirtual *base = &d;
  base->f();
  assert(base_calls == 0 && first_calls == 0 && second_calls == 2);

  OverridesWithoutKeyword m;
  base = &m;
  base->f();
  assert(base_calls == 0 && first_calls == 1 && second_calls == 2);
  return 0;
}
