// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.dynamic.cast]/7: if T is pointer to cv void, the result is a
// pointer to the most derived object pointed to by v.
#include <cassert>

struct FirstBase {
  int first = 1;
  virtual void f() {}
};

struct SecondBase {
  int second = 2;
  virtual void g() {}
};

struct MostDerived : FirstBase, SecondBase {};

int main() {
  MostDerived d;
  SecondBase *second = &d;

  void *mostDerived = dynamic_cast<void *>(second);
  void *addressOfD = &d;
  void *addressOfSecond = second;
  assert(mostDerived == addressOfD);
  assert(mostDerived != addressOfSecond);
  return 0;
}
