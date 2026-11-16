// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.prim.this]/1: the keyword this names a pointer to the object for
// which the member function is invoked.
#include <cassert>

struct DeclaresVirtual {
  int total = 0;
  virtual void add(int amount) = 0;
};

struct AddsTwice : DeclaresVirtual {
  int calls = 0;
  void add(int amount) override {
    total += 2 * amount;
    ++calls;
  }
};

int main() {
  AddsTwice d;
  DeclaresVirtual *base = &d;
  AddsTwice *derived = &d;

  base->add(3);
  assert(d.total == 6 && d.calls == 1);
  assert(derived->total == 6);

  base->add(1);
  assert(base->total == 8);
  assert(derived->calls == 2);
  return 0;
}
