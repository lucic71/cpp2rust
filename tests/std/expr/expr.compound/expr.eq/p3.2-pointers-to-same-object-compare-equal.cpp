// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.eq]/3.2: after pointer conversions bring the operands to their
// composite pointer type, two pointers compare equal if both are null or both
// represent the same address.
#include <cassert>

struct FirstBase {
  int first = 1;
};

struct SecondBase {
  int second = 2;
};

struct HasTwoBases : FirstBase, SecondBase {};

int main() {
  HasTwoBases d;
  HasTwoBases *derived = &d;
  FirstBase *first = &d;
  SecondBase *second = &d;

  assert(first == derived);
  assert(second == derived);
  assert(!(second != derived));

  SecondBase *secondAgain = derived;
  assert(secondAgain == second);

  HasTwoBases *nullDerived = nullptr;
  SecondBase *nullSecond = nullDerived;
  assert(nullSecond == nullptr);
  assert(nullSecond == nullDerived);
  return 0;
}
