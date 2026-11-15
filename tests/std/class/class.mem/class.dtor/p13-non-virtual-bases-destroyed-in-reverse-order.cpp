// panic: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.dtor]/13: after executing its body, a destructor calls the
// destructors for the class's non-static data members and then for its
// non-virtual direct base classes, in the reverse order of the completion of
// their constructors. Each base destructor does the same for its own bases,
// so the indirect base is destroyed last.
#include <cassert>

static int sequence;
static int indirect_base_stamp, first_base_stamp, second_base_stamp;
static int first_member_stamp, second_member_stamp, body_stamp;

struct StampsDestruction {
  int *stamp;
  StampsDestruction(int *s) : stamp(s) {}
  ~StampsDestruction() { *stamp = ++sequence; }
};

struct IndirectBase {
  ~IndirectBase() { indirect_base_stamp = ++sequence; }
};

struct FirstBase : IndirectBase {
  ~FirstBase() { first_base_stamp = ++sequence; }
};

struct SecondBase {
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
  assert(indirect_base_stamp == 6);
  assert(sequence == 6);
  return 0;
}
