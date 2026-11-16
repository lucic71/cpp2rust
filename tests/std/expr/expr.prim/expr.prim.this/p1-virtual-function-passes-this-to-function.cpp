// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.prim.this]/1: the keyword this names a pointer to the object for
// which the member function is invoked.
#include <cassert>

static int f_calls;

struct DeclaresVirtual {
  int value = 0;
  virtual void f() = 0;
};

static void setValueThroughPointer(DeclaresVirtual *object, int value) {
  object->value = value;
}

struct PassesThis : DeclaresVirtual {
  void f() override {
    ++f_calls;
    setValueThroughPointer(this, 7);
  }
};

int main() {
  PassesThis d;
  DeclaresVirtual *base = &d;

  base->f();
  assert(f_calls == 1);
  assert(d.value == 7);
  assert(base->value == 7);
  return 0;
}
