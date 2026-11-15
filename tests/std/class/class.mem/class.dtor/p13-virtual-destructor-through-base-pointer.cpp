// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.dtor]/13: a destructor calls the destructors of its bases as if
// they were referenced with a qualified name, ignoring any overriding
// destructors in more derived classes. Deleting a derived object through a
// base pointer runs the most derived destructor, and each base destructor
// then runs exactly once.
#include <cassert>

static int sequence;
static int base_stamp, first_stamp, second_stamp;
static int base_dtors, first_dtors, second_dtors;

struct HasVirtualDestructor {
  virtual ~HasVirtualDestructor() {
    ++base_dtors;
    base_stamp = ++sequence;
  }
};

struct OverridesDestructor : HasVirtualDestructor {
  ~OverridesDestructor() override {
    ++first_dtors;
    first_stamp = ++sequence;
  }
};

struct OverridesDestructorAgain : OverridesDestructor {
  ~OverridesDestructorAgain() override {
    ++second_dtors;
    second_stamp = ++sequence;
  }
};

int main() {
  HasVirtualDestructor *p = new OverridesDestructorAgain;
  delete p;
  assert(second_dtors == 1 && first_dtors == 1 && base_dtors == 1);
  assert(second_stamp == 1);
  assert(first_stamp == 2);
  assert(base_stamp == 3);

  p = new OverridesDestructor;
  delete p;
  assert(second_dtors == 1 && first_dtors == 2 && base_dtors == 2);
  assert(first_stamp == 4);
  assert(base_stamp == 5);
  return 0;
}
