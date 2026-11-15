// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.virtual]/8.1: covariant return types can both be lvalue
// references to classes. The override returns a reference to the derived
// class; through the base it is seen as a reference to the base.
#include <cassert>

static int base_calls, derived_calls;

struct ReturnsBaseReference {
  int baseValue = 0;
  virtual ReturnsBaseReference &self() {
    ++base_calls;
    return *this;
  }
};

struct ReturnsDerivedReference : ReturnsBaseReference {
  int derivedOnly = 0;
  ReturnsDerivedReference &self() override {
    ++derived_calls;
    return *this;
  }
};

int main() {
  ReturnsDerivedReference d;
  ReturnsBaseReference &base = d;
  base.self().baseValue = 3;
  assert(base_calls == 0 && derived_calls == 1);
  assert(d.baseValue == 3);

  d.self().derivedOnly = 4;
  d.self().baseValue = 5;
  assert(base_calls == 0 && derived_calls == 3);
  assert(d.derivedOnly == 4);
  assert(base.baseValue == 5);
  return 0;
}
