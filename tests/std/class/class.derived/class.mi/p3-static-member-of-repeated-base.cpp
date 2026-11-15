// panic: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.mi]/3, note 3 and example 2: a class can be an indirect base
// class more than once, and both a direct and an indirect base class. Its
// static members and enumerators can still be referred to unambiguously
// through the derived class.
#include <cassert>

struct RepeatedBase {
  static inline int ctorCount = 0;
  static inline int touchCount = 0;
  enum Kind { Plain = 7 };

  int value = 0;

  RepeatedBase() { ++ctorCount; }
  static void touch() { ++touchCount; }
};

struct FirstPath : RepeatedBase {};
struct SecondPath : RepeatedBase {};

struct HasTwoRepeatedBases : FirstPath, SecondPath {
  void touchFromMember() { touch(); }
};

struct DirectAndIndirect : FirstPath, RepeatedBase {};

int main() {
  HasTwoRepeatedBases c;
  assert(RepeatedBase::ctorCount == 2);
  HasTwoRepeatedBases::touch();
  assert(RepeatedBase::touchCount == 1);
  c.touchFromMember();
  assert(RepeatedBase::touchCount == 2);
  assert(HasTwoRepeatedBases::touchCount == 2);
  assert(HasTwoRepeatedBases::Plain == 7);

  DirectAndIndirect d;
  assert(DirectAndIndirect::ctorCount == 4);
  DirectAndIndirect::touch();
  assert(RepeatedBase::touchCount == 3);
  return 0;
}
