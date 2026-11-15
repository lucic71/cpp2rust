// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.abstract]/2: a pure virtual function need be defined only if
// called with, or as if with, the qualified-id syntax. The destructor of a
// derived class calls the base destructor that way, so a pure virtual
// destructor is defined and runs for every derived object.
#include <cassert>

static int abstract_dtors, concrete_dtors;

struct HasPureVirtualDestructor {
  virtual ~HasPureVirtualDestructor() = 0;
};

HasPureVirtualDestructor::~HasPureVirtualDestructor() { ++abstract_dtors; }

struct OverridesDestructor : HasPureVirtualDestructor {
  ~OverridesDestructor() override { ++concrete_dtors; }
};

int main() {
  {
    OverridesDestructor d;
    assert(abstract_dtors == 0 && concrete_dtors == 0);
  }
  assert(abstract_dtors == 1 && concrete_dtors == 1);

  HasPureVirtualDestructor *p = new OverridesDestructor;
  delete p;
  assert(abstract_dtors == 2 && concrete_dtors == 2);
  return 0;
}
