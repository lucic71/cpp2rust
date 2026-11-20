// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [expr.new]/23.1: without a new-initializer the object is
// default-initialized, which calls the default constructor of a class and of
// each element of an array.
namespace p23_1_new_without_initializer {
int constructed = 0;

struct Counted {
  int id;
  Counted() : id(++constructed) {}
};

void test() {
  Counted *one = new Counted;
  assert(constructed == 1 && one->id == 1);
  delete one;
  Counted *many = new Counted[3];
  assert(constructed == 4);
  assert(many[0].id == 2 && many[1].id == 3 && many[2].id == 4);
  delete[] many;
}
} // namespace p23_1_new_without_initializer

// N4950 [expr.new]/23.2: a new-initializer initializes the object as a
// direct-initialization does: () value-initializes, arguments select a
// constructor, and braces initialize an aggregate.
namespace p23_2_new_with_initializer {
struct Pair {
  int first;
  int second;
};

struct Tracker {
  Tracker *self;
  int value;
  Tracker(int value) : self(this), value(value) {}
};

void test() {
  Pair *dirty = new Pair;
  dirty->first = 0x5a5a;
  dirty->second = 0x5a5a;
  delete dirty;
  Pair *zeroed = new Pair();
  assert(zeroed->first == 0 && zeroed->second == 0);
  delete zeroed;

  Tracker *tracker = new Tracker(4);
  assert(tracker->self == tracker && tracker->value == 4);
  delete tracker;

  Pair *braces = new Pair{1, 2};
  assert(braces->first == 1 && braces->second == 2);
  delete braces;

  int *number = new int(7);
  assert(*number == 7);
  delete number;
  int *zero = new int();
  assert(*zero == 0);
  delete zero;
}
} // namespace p23_2_new_with_initializer

int main() {
  p23_1_new_without_initializer::test();
  p23_2_new_with_initializer::test();
  return 0;
}
