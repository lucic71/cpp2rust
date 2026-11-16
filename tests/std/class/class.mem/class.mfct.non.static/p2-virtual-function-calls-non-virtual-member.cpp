// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.mfct.non.static]/2: inside a member function, a name that
// resolves to a non-static member of the class or of a base class is
// transformed into a class member access expression using (*this).
#include <cassert>

struct DeclaresVirtual {
  int value = 0;
  void setValue(int v) { value = v; }
  virtual void f() = 0;
};

struct CallsNonVirtualMembers : DeclaresVirtual {
  int own = 0;
  void setOwn(int v) { own = v; }
  void f() override {
    setValue(1);
    setOwn(2);
  }
};

int main() {
  CallsNonVirtualMembers d;
  DeclaresVirtual *base = &d;
  assert(d.value == 0 && d.own == 0);

  base->f();
  assert(d.value == 1 && d.own == 2);
  return 0;
}
