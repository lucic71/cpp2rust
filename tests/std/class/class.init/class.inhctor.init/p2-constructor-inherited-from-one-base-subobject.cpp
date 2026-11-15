// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.inhctor.init]/2: a constructor may be inherited from only one
// base class subobject of a type. JoinsInheritingAndPlain inherits
// HasTwoConstructors(int) only through InheritsConstructors, so that subobject
// gets the int constructor and the one inside DoesNotInherit the default
// constructor. JoinsVirtualPaths inherits through a single virtual base, which
// is initialized once.
#include <cassert>

static int default_ctors, int_ctors, shared_int_ctors;

struct HasTwoConstructors {
  int value;
  HasTwoConstructors() : value(-1) { ++default_ctors; }
  HasTwoConstructors(int v) : value(v) { ++int_ctors; }
};

struct InheritsConstructors : HasTwoConstructors {
  using HasTwoConstructors::HasTwoConstructors;
};

struct DoesNotInherit : HasTwoConstructors {};

struct JoinsInheritingAndPlain : InheritsConstructors, DoesNotInherit {
  using InheritsConstructors::InheritsConstructors;
  using DoesNotInherit::DoesNotInherit;
};

struct HasIntConstructor {
  int value;
  HasIntConstructor(int v) : value(v) { ++shared_int_ctors; }
};

struct SharedBase : HasIntConstructor {
  using HasIntConstructor::HasIntConstructor;
};

struct FirstVirtualPath : virtual SharedBase {
  using SharedBase::SharedBase;
};

struct SecondVirtualPath : virtual SharedBase {
  using SharedBase::SharedBase;
};

struct JoinsVirtualPaths : FirstVirtualPath, SecondVirtualPath {
  using FirstVirtualPath::FirstVirtualPath;
  using SecondVirtualPath::SecondVirtualPath;
};

int main() {
  JoinsInheritingAndPlain joined(0);
  assert(int_ctors == 1 && default_ctors == 1);
  assert(joined.InheritsConstructors::value == 0);
  assert(joined.DoesNotInherit::value == -1);

  JoinsVirtualPaths shared(5);
  assert(shared_int_ctors == 1);
  assert(shared.value == 5);
  return 0;
}
