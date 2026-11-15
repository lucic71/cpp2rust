// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.inhctor.init]/1: when an inherited constructor initializes a
// derived object, the base subobject it was inherited from is initialized by
// that constructor and the rest as if by a defaulted default constructor:
// other bases are default-initialized, and members use their default member
// initializers after the bases.
#include <cassert>

static int sequence;

static int nextStamp() { return ++sequence; }

struct HasIntConstructor {
  int value, stamp;
  HasIntConstructor(int v) : value(v), stamp(nextStamp()) {}
};

struct HasDefaultConstructor {
  int stamp;
  HasDefaultConstructor() : stamp(nextStamp()) {}
};

struct InheritsConstructor : HasIntConstructor {
  using HasIntConstructor::HasIntConstructor;
  int memberStamp = nextStamp();
};

struct InheritsConstructorWithOtherBase : HasDefaultConstructor,
                                          HasIntConstructor {
  using HasIntConstructor::HasIntConstructor;
  int memberStamp = nextStamp();
};

int main() {
  InheritsConstructor d(2);
  assert(d.value == 2);
  assert(d.HasIntConstructor::stamp == 1);
  assert(d.memberStamp == 2);

  sequence = 0;
  InheritsConstructorWithOtherBase e(3);
  assert(e.value == 3);
  assert(e.HasDefaultConstructor::stamp == 1);
  assert(e.HasIntConstructor::stamp == 2);
  assert(e.memberStamp == 3);
  return 0;
}
