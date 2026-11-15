// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.mi]/1: a class can be derived from any number of base classes.
// HasThreeBases has one subobject of each base; a pointer to each base reaches
// only that base's members.
#include <cassert>

struct CountsFirst {
  int first = 0;
  void touchFirst() { ++first; }
};

struct CountsSecond {
  int second = 0;
  void touchSecond() { ++second; }
};

struct CountsThird {
  int third = 0;
  void touchThird() { ++third; }
};

struct HasThreeBases : CountsFirst, CountsSecond, CountsThird {
  void touchAll() {
    touchFirst();
    touchSecond();
    touchThird();
  }
};

int main() {
  HasThreeBases d;
  d.touchAll();
  assert(d.first == 1 && d.second == 1 && d.third == 1);
  d.touchSecond();
  assert(d.first == 1 && d.second == 2 && d.third == 1);

  CountsFirst *first = &d;
  first->touchFirst();
  assert(d.first == 2 && d.second == 2 && d.third == 1);

  CountsSecond *second = &d;
  second->touchSecond();
  assert(d.first == 2 && d.second == 3 && d.third == 1);

  CountsThird *third = &d;
  third->touchThird();
  assert(d.first == 2 && d.second == 3 && d.third == 2);

  assert(first->first == 2);
  assert(second->second == 3);
  assert(third->third == 2);
  return 0;
}
