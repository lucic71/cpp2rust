// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.dynamic.cast]/9: the value of a failed cast to pointer type is
// the null pointer value of the required result type.
#include <cassert>

struct Base {
  virtual void f() {}
};

struct FirstDerived : Base {};
struct SecondDerived : Base {};

int main() {
  Base onlyBase;
  Base *base = &onlyBase;
  assert(dynamic_cast<FirstDerived *>(base) == nullptr);

  SecondDerived second;
  base = &second;
  assert(dynamic_cast<FirstDerived *>(base) == nullptr);
  assert(dynamic_cast<SecondDerived *>(base) == &second);
  return 0;
}
