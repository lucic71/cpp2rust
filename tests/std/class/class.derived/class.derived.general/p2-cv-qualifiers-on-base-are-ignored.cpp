// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.derived.general]/2: a class-or-decltype denotes a possibly
// cv-qualified class type; any cv-qualifiers are ignored. The base named
// through a const typedef is an ordinary, modifiable HasCounter subobject.
#include <cassert>

struct HasCounter {
  int count = 0;
  void increment() { ++count; }
};

typedef const HasCounter ConstHasCounter;

struct DerivesFromConstTypedef : ConstHasCounter {
  void incrementTwice() {
    increment();
    increment();
  }
};

int main() {
  DerivesFromConstTypedef d;
  d.increment();
  assert(d.count == 1);
  d.incrementTwice();
  assert(d.count == 3);
  HasCounter *p = &d;
  p->increment();
  assert(d.count == 4);
  return 0;
}
