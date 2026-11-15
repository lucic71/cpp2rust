// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.base.init]/13.1: first, and only for the constructor of the
// most derived class, virtual base classes are initialized in the order they
// appear on a depth-first left-to-right traversal of the graph of base
// classes. The traversal of MostDerived reaches VirtualOfLeft, then
// VirtualOfRight, then DirectVirtual, all before any non-virtual base.
#include <cassert>

static int sequence;

struct StampsConstruction {
  int stamp;
  StampsConstruction() : stamp(++sequence) {}
};

struct VirtualOfLeft : StampsConstruction {};
struct VirtualOfRight : StampsConstruction {};
struct DirectVirtual : StampsConstruction {};
struct NonVirtualFirst : StampsConstruction {};

struct Left : virtual VirtualOfLeft {
  int leftStamp;
  Left() : leftStamp(++sequence) {}
};

struct Right : virtual VirtualOfRight, virtual VirtualOfLeft {
  int rightStamp;
  Right() : rightStamp(++sequence) {}
};

struct MostDerived : NonVirtualFirst, Left, Right, virtual DirectVirtual {
  int ownStamp;
  MostDerived() : ownStamp(++sequence) {}
};

int main() {
  MostDerived d;
  assert(d.VirtualOfLeft::stamp == 1);
  assert(d.VirtualOfRight::stamp == 2);
  assert(d.DirectVirtual::stamp == 3);
  assert(d.NonVirtualFirst::stamp == 4);
  assert(d.leftStamp == 5);
  assert(d.rightStamp == 6);
  assert(d.ownStamp == 7);
  assert(sequence == 7);

  sequence = 0;
  Right r;
  assert(r.VirtualOfRight::stamp == 1);
  assert(r.VirtualOfLeft::stamp == 2);
  assert(r.rightStamp == 3);
  return 0;
}
