// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.eq]/3.3: pointers that are not both null and do not represent
// the same address compare unequal.
#include <cassert>

struct FirstBase {
  int first = 1;
};

struct SecondBase {
  int second = 2;
};

struct HasTwoBases : FirstBase, SecondBase {};

int main() {
  HasTwoBases a;
  HasTwoBases b;

  SecondBase *secondOfA = &a;
  SecondBase *secondOfB = &b;
  assert(secondOfA != secondOfB);
  assert(!(secondOfA == secondOfB));

  FirstBase *firstOfA = &a;
  void *firstAddress = firstOfA;
  void *secondAddress = secondOfA;
  assert(firstAddress != secondAddress);

  SecondBase *nullSecond = nullptr;
  assert(secondOfA != nullSecond);
  return 0;
}
