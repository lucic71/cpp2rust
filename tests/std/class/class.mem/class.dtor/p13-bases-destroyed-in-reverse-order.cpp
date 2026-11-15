// panic: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.dtor]/13: after executing its body, a destructor calls the
// destructors for the class's non-static data members, then for its
// non-virtual direct base classes and, if the class is the most derived
// class, for its virtual base classes; bases and members are destroyed in the
// reverse order of the completion of their constructors.
#include <cassert>

static int sequence;
static int virtual_base_stamp, first_base_stamp, second_base_stamp;
static int first_member_stamp, second_member_stamp, body_stamp;

struct StampsDestruction {
  int *stamp;
  StampsDestruction(int *s) : stamp(s) {}
  ~StampsDestruction() { *stamp = ++sequence; }
};

struct VirtualBase {
  ~VirtualBase() { virtual_base_stamp = ++sequence; }
};

struct FirstBase : virtual VirtualBase {
  ~FirstBase() { first_base_stamp = ++sequence; }
};

struct SecondBase : virtual VirtualBase {
  ~SecondBase() { second_base_stamp = ++sequence; }
};

struct HasBasesAndMembers : FirstBase, SecondBase {
  StampsDestruction firstMember;
  StampsDestruction secondMember;
  HasBasesAndMembers()
      : firstMember(&first_member_stamp), secondMember(&second_member_stamp) {}
  ~HasBasesAndMembers() { body_stamp = ++sequence; }
};

int main() {
  {
    HasBasesAndMembers d;
    assert(sequence == 0);
  }
  assert(body_stamp == 1);
  assert(second_member_stamp == 2);
  assert(first_member_stamp == 3);
  assert(second_base_stamp == 4);
  assert(first_base_stamp == 5);
  assert(virtual_base_stamp == 6);
  assert(sequence == 6);
  return 0;
}
