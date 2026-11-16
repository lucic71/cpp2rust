// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [intro.object]/9: two objects with overlapping lifetimes that are not
// nested within each other have distinct addresses and occupy disjoint bytes
// of storage.
#include <cassert>

struct FirstBase {
  int first = 1;
  virtual void f() {}
};

struct SecondBase {
  int second = 2;
  virtual void g() {}
};

struct HasTwoBases : FirstBase, SecondBase {};

int main() {
  HasTwoBases d;
  FirstBase *first = &d;
  SecondBase *second = &d;

  void *firstAddress = first;
  void *secondAddress = second;
  assert(firstAddress != secondAddress);

  first->first = 10;
  assert(d.first == 10 && d.second == 2);
  second->second = 20;
  assert(d.first == 10 && d.second == 20);
  return 0;
}
