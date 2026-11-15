// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.abstract]/2: a class with a pure virtual function is abstract
// and exists only as a base class subobject. A non-virtual member of the
// abstract class calls the pure virtual function, which runs the overrider of
// the derived class the object belongs to.
#include <cassert>

static int first_calls, second_calls;

struct HasPureVirtual {
  void callPure() { pure(); }
  virtual void pure() = 0;
};

struct FirstOverrider : HasPureVirtual {
  void pure() override { ++first_calls; }
};

struct SecondOverrider : HasPureVirtual {
  void pure() override { ++second_calls; }
};

int main() {
  FirstOverrider first;
  SecondOverrider second;

  HasPureVirtual *p = &first;
  p->callPure();
  assert(first_calls == 1 && second_calls == 0);

  p = &second;
  p->callPure();
  assert(first_calls == 1 && second_calls == 1);
  p->pure();
  assert(first_calls == 1 && second_calls == 2);
  return 0;
}
