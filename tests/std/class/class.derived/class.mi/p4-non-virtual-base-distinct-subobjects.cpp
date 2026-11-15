// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.mi]/4 (figure 4): for each distinct occurrence of a non-virtual
// base class in the class lattice, the most derived object contains a distinct
// base class subobject. A HasTwoRepeatedBases object has two RepeatedBase
// subobjects, one in FirstPath and one in SecondPath, selected by explicit
// qualification.
#include <cassert>

static int repeated_base_ctors;

struct RepeatedBase {
  int value;
  RepeatedBase() : value(0) { ++repeated_base_ctors; }
};

struct FirstPath : RepeatedBase {};
struct SecondPath : RepeatedBase {};

struct HasTwoRepeatedBases : FirstPath, SecondPath {
  void copyValueFromSecondToFirst() { FirstPath::value = SecondPath::value; }
};

int main() {
  HasTwoRepeatedBases d;
  assert(repeated_base_ctors == 2);
  d.FirstPath::value = 1;
  d.SecondPath::value = 2;
  assert(d.FirstPath::value == 1);
  assert(d.SecondPath::value == 2);

  FirstPath *firstPath = &d;
  SecondPath *secondPath = &d;
  RepeatedBase *baseOfFirst = firstPath;
  RepeatedBase *baseOfSecond = secondPath;
  assert(baseOfFirst != baseOfSecond);
  assert(baseOfFirst->value == 1);
  assert(baseOfSecond->value == 2);

  d.copyValueFromSecondToFirst();
  assert(baseOfFirst->value == 2);
  assert(baseOfSecond->value == 2);
  baseOfSecond->value = 3;
  assert(d.FirstPath::value == 2);
  assert(d.SecondPath::value == 3);
  return 0;
}
