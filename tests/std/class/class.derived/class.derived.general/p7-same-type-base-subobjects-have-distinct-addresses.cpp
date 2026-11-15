// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.derived.general]/7, note 5: a base class subobject can be of
// zero size, but two subobjects that have the same class type and belong to
// the same most derived object are not allocated at the same address.
#include <cassert>

struct Empty {};

struct FirstPath : Empty {};
struct SecondPath : Empty {};

struct HasTwoEmptyBases : FirstPath, SecondPath {};

int main() {
  HasTwoEmptyBases d;
  FirstPath *first = &d;
  SecondPath *second = &d;
  Empty *throughFirst = first;
  Empty *throughSecond = second;
  assert(throughFirst != throughSecond);
  return 0;
}
