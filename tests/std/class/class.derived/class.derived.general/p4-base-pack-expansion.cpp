// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.derived.general]/4: a base-specifier followed by an ellipsis is
// a pack expansion. DerivesFromPack<CountsFirst, CountsSecond> has one base
// class subobject per pack element.
#include <cassert>

struct CountsFirst {
  int first = 0;
  void touchFirst() { ++first; }
};

struct CountsSecond {
  int second = 0;
  void touchSecond() { ++second; }
};

template <class... Mixins> struct DerivesFromPack : Mixins... {
  int own = 0;
};

int main() {
  DerivesFromPack<CountsFirst, CountsSecond> both;
  both.touchFirst();
  assert(both.first == 1 && both.second == 0);
  both.touchSecond();
  assert(both.first == 1 && both.second == 1);
  CountsSecond *second = &both;
  second->touchSecond();
  assert(both.first == 1 && both.second == 2);

  DerivesFromPack<CountsFirst> one;
  one.touchFirst();
  assert(one.first == 1);
  assert(both.first == 1);

  DerivesFromPack<> none;
  none.own = 1;
  assert(none.own == 1);
  assert(both.own == 0);
  return 0;
}
