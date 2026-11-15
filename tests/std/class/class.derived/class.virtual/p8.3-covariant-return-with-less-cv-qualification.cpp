// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.virtual]/8.3: in a covariant return type the class type of the
// overrider has the same cv-qualification as or less than that of the
// overridden function. The base returns a pointer to const; the override
// returns a pointer to non-const through which the object can be modified.
#include <cassert>

static int base_calls, derived_calls;

struct ReturnsPointerToConst {
  int value = 1;
  virtual const ReturnsPointerToConst *self() {
    ++base_calls;
    return this;
  }
};

struct ReturnsPointerToNonConst : ReturnsPointerToConst {
  ReturnsPointerToNonConst *self() override {
    ++derived_calls;
    return this;
  }
};

int main() {
  ReturnsPointerToNonConst d;
  ReturnsPointerToConst *base = &d;
  const ReturnsPointerToConst *fromBase = base->self();
  assert(base_calls == 0 && derived_calls == 1);
  assert(fromBase->value == 1);

  d.self()->value = 2;
  assert(base_calls == 0 && derived_calls == 2);
  assert(fromBase->value == 2);
  return 0;
}
