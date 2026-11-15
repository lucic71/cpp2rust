// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.derived.general]/2, note 2: a pointer to a derived class
// converts to a pointer to an accessible unambiguous base class, and a derived
// lvalue binds to a reference to such a base. Both designate the base class
// subobject of the same object.
#include <cassert>

struct HasCounter {
  int count = 0;
  void increment() { ++count; }
};

struct DerivesDirectly : HasCounter {
  int own = 0;
};

struct DerivesIndirectly : DerivesDirectly {};

static void incrementThroughPointer(HasCounter *p) { p->increment(); }
static void incrementThroughReference(HasCounter &r) { r.increment(); }

int main() {
  DerivesDirectly d;
  incrementThroughPointer(&d);
  assert(d.count == 1);
  incrementThroughReference(d);
  assert(d.count == 2);
  assert(d.own == 0);

  HasCounter *p = &d;
  HasCounter &r = d;
  assert(p == &r);
  p->increment();
  assert(d.count == 3);
  r.increment();
  assert(d.count == 4);
  d.increment();
  assert(p->count == 5);
  assert(r.count == 5);

  DerivesIndirectly i;
  incrementThroughPointer(&i);
  assert(i.count == 1);
  incrementThroughReference(i);
  assert(i.count == 2);
  DerivesDirectly *asDirectBase = &i;
  asDirectBase->increment();
  assert(i.count == 3);
  assert(d.count == 5);
  return 0;
}
