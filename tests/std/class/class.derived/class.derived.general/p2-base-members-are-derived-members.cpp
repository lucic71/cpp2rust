// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.derived.general]/2: members of a base class are also members of
// the derived class. A data member and a member function of HasCounter are
// used as members of a directly and of an indirectly derived class.
#include <cassert>

struct HasCounter {
  int count = 0;
  void increment() { ++count; }
};

struct DerivesDirectly : HasCounter {
  int own = 0;
  void incrementBoth() {
    increment();
    ++own;
  }
};

struct DerivesIndirectly : DerivesDirectly {
  void incrementTwice() {
    increment();
    incrementBoth();
  }
};

int main() {
  DerivesDirectly d;
  d.increment();
  assert(d.count == 1 && d.own == 0);
  d.incrementBoth();
  assert(d.count == 2 && d.own == 1);
  d.count += 5;
  assert(d.count == 7);
  d.increment();
  assert(d.count == 8);

  DerivesIndirectly i;
  i.incrementTwice();
  assert(i.count == 2 && i.own == 1);
  i.increment();
  assert(i.count == 3 && i.own == 1);
  assert(d.count == 8);
  return 0;
}
