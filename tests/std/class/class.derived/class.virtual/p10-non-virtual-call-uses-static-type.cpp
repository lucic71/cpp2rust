// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.virtual]/10: a virtual call depends on the dynamic type of the
// object, a non-virtual call only on the static type of the pointer.
// overridden() is overridden, hiddenByOverload() is hidden by a non-virtual
// hiddenByOverload(int), and nonVirtual() is redeclared but not virtual.
#include <cassert>

static int base_overridden, base_hidden, base_non_virtual;
static int derived_overridden, derived_overload, derived_non_virtual;

struct DeclaresFunctions {
  virtual void overridden() { ++base_overridden; }
  virtual void hiddenByOverload() { ++base_hidden; }
  void nonVirtual() { ++base_non_virtual; }
};

struct RedeclaresFunctions : DeclaresFunctions {
  void overridden() override { ++derived_overridden; }
  void hiddenByOverload(int) { ++derived_overload; }
  void nonVirtual() { ++derived_non_virtual; }
};

int main() {
  RedeclaresFunctions d;

  DeclaresFunctions *base = &d;
  base->overridden();
  assert(base_overridden == 0 && derived_overridden == 1);
  base->hiddenByOverload();
  assert(base_hidden == 1 && derived_overload == 0);
  base->nonVirtual();
  assert(base_non_virtual == 1 && derived_non_virtual == 0);

  RedeclaresFunctions *derived = &d;
  derived->overridden();
  assert(base_overridden == 0 && derived_overridden == 2);
  derived->hiddenByOverload(0);
  assert(base_hidden == 1 && derived_overload == 1);
  derived->nonVirtual();
  assert(base_non_virtual == 1 && derived_non_virtual == 1);
  return 0;
}
