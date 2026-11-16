// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.dynamic.cast]/9: a failed cast to reference type throws an
// exception of a type that would match a handler of type std::bad_cast.
#include <cassert>
#include <typeinfo>

static int successful_casts, failed_casts;

struct Base {
  virtual void f() {}
};

struct FirstDerived : Base {};
struct SecondDerived : Base {};

static void castToFirstDerived(Base &base) {
  try {
    FirstDerived &derived = dynamic_cast<FirstDerived &>(base);
    (void)derived;
    ++successful_casts;
  } catch (const std::bad_cast &) {
    ++failed_casts;
  }
}

int main() {
  FirstDerived first;
  castToFirstDerived(first);
  assert(successful_casts == 1 && failed_casts == 0);

  SecondDerived second;
  castToFirstDerived(second);
  assert(successful_casts == 1 && failed_casts == 1);
  return 0;
}
