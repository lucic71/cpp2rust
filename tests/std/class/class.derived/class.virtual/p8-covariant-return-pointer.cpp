// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.virtual]/8: the return type of an overriding function is
// identical to or covariant with that of the overridden function (8.1: both
// pointers to classes; 8.2: the base's class is a base of the derived's
// class). The override returns a pointer to the derived class.
#include <cassert>

static int base_calls, derived_calls;

struct ReturnsBasePointer {
  virtual ReturnsBasePointer *self() {
    ++base_calls;
    return this;
  }
};

struct ReturnsDerivedPointer : ReturnsBasePointer {
  int derivedOnly = 0;
  ReturnsDerivedPointer *self() override {
    ++derived_calls;
    return this;
  }
};

int main() {
  ReturnsDerivedPointer d;
  ReturnsBasePointer *base = &d;
  ReturnsBasePointer *fromBase = base->self();
  assert(base_calls == 0 && derived_calls == 1);
  assert(fromBase == base);

  ReturnsDerivedPointer *fromDerived = d.self();
  assert(base_calls == 0 && derived_calls == 2);
  fromDerived->derivedOnly = 5;
  assert(d.derivedOnly == 5);

  ReturnsBasePointer b;
  assert(b.self() == &b);
  assert(base_calls == 1 && derived_calls == 2);
  return 0;
}
