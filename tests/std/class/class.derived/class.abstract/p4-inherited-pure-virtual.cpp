// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.abstract]/4: a class is abstract if it has at least one pure
// virtual function for which the final overrider is pure virtual.
// OverridesFirstOnly leaves second() pure, so it is still abstract;
// OverridesSecond overrides it and inherits the overrider of first().
#include <cassert>

static int first_calls, second_calls;

struct HasTwoPureVirtuals {
  virtual void first() = 0;
  virtual void second() = 0;
};

struct OverridesFirstOnly : HasTwoPureVirtuals {
  void first() override { ++first_calls; }
};

struct OverridesSecond : OverridesFirstOnly {
  void second() override { ++second_calls; }
};

int main() {
  OverridesSecond d;
  HasTwoPureVirtuals *base = &d;
  base->first();
  assert(first_calls == 1 && second_calls == 0);
  base->second();
  assert(first_calls == 1 && second_calls == 1);

  OverridesFirstOnly *stillAbstract = &d;
  stillAbstract->first();
  assert(first_calls == 2 && second_calls == 1);
  stillAbstract->second();
  assert(first_calls == 2 && second_calls == 2);
  return 0;
}
