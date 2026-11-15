// panic-ub: refcount
// nondet-result: unsafe
// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.delete]/3: if the static type of the object to be deleted
// differs from its dynamic type and the static type is not a base class with
// a virtual destructor, the behavior is undefined. A derived object is deleted
// through a pointer to its base, whose destructor is not virtual.

static int base_dtors, derived_dtors;

struct HasNonVirtualDestructor {
  ~HasNonVirtualDestructor() { ++base_dtors; }
};

struct DerivesWithOwnDestructor : HasNonVirtualDestructor {
  ~DerivesWithOwnDestructor() { ++derived_dtors; }
};

int main() {
  HasNonVirtualDestructor *p = new DerivesWithOwnDestructor;
  delete p;
  return base_dtors + 2 * derived_dtors;
}
