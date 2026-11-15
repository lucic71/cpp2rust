// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.base.init]/13.3 and /13.4: after the base classes, non-static
// data members are initialized in the order they were declared in the class
// definition, regardless of the order of the mem-initializers; finally the
// body of the constructor is executed.
#include <cassert>

static int sequence;

struct StampsConstruction {
  int stamp;
  StampsConstruction() : stamp(++sequence) {}
  StampsConstruction(int) : stamp(++sequence) {}
};

struct FirstBase : StampsConstruction {};
struct SecondBase : StampsConstruction {};

struct InitializesMembersOutOfOrder : FirstBase, SecondBase {
  StampsConstruction firstMember;
  StampsConstruction secondMember;
  StampsConstruction thirdMember = StampsConstruction(0);
  int bodyStamp;

  InitializesMembersOutOfOrder()
      : secondMember(0), SecondBase(), firstMember(0), FirstBase() {
    bodyStamp = ++sequence;
  }
};

int main() {
  InitializesMembersOutOfOrder d;
  assert(d.FirstBase::stamp == 1);
  assert(d.SecondBase::stamp == 2);
  assert(d.firstMember.stamp == 3);
  assert(d.secondMember.stamp == 4);
  assert(d.thirdMember.stamp == 5);
  assert(d.bodyStamp == 6);
  assert(sequence == 6);
  return 0;
}
