// panic-ub: refcount
// nondet-result: unsafe
// translation-fail: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.cdtor]/5: if the operand of typeid refers to the object under
// construction and its static type is neither the constructor's class nor one
// of its bases, the behavior is undefined. The constructor applies typeid to
// a pointer to its sibling base.
#include <typeinfo>

static int sibling_matches, most_derived_matches;

struct VirtualBase {
  virtual void f() {}
};

struct Sibling : virtual VirtualBase {};

struct MostDerived;

struct AppliesTypeidToSiblingInConstructor : virtual VirtualBase {
  AppliesTypeidToSiblingInConstructor(Sibling *sibling);
};

struct MostDerived : Sibling, AppliesTypeidToSiblingInConstructor {
  MostDerived() : AppliesTypeidToSiblingInConstructor(this) {}
};

AppliesTypeidToSiblingInConstructor::AppliesTypeidToSiblingInConstructor(
    Sibling *sibling) {
  if (typeid(*sibling) == typeid(Sibling)) {
    ++sibling_matches;
  }
  if (typeid(*sibling) == typeid(MostDerived)) {
    ++most_derived_matches;
  }
}

int main() {
  MostDerived d;
  return sibling_matches + 2 * most_derived_matches;
}
