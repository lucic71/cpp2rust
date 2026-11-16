// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.mptr.oper]/2: the second operand of .* is a pointer to member of
// T, and the first is a glvalue of class T or of a class of which T is an
// unambiguous and accessible base class.
#include <cassert>

struct FirstBase {
  int first = 1;
};

struct SecondBase {
  int second = 2;
  int other = 3;
};

struct HasTwoBases : FirstBase, SecondBase {};

int main() {
  HasTwoBases d;
  int SecondBase::*member = &SecondBase::second;
  assert(d.*member == 2);

  d.*member = 20;
  assert(d.second == 20);
  assert(d.first == 1 && d.other == 3);

  member = &SecondBase::other;
  HasTwoBases *pointer = &d;
  pointer->*member = 30;
  assert(d.other == 30);
  assert(d.first == 1 && d.second == 20);
  return 0;
}
