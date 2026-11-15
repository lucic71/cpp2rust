// panic-ub: refcount
// nondet-result: unsafe
// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.base.init]/16: calling a member function in a ctor-initializer
// before all the mem-initializers for base classes have completed is
// undefined. The argument for the base calls readOwn(), which reads a member
// that is initialized only after the base.

struct HasIntConstructor {
  int value;
  HasIntConstructor(int v) : value(v) {}
};

struct CallsMemberBeforeBaseIsInitialized : HasIntConstructor {
  int own = 5;
  int readOwn() { return own; }
  CallsMemberBeforeBaseIsInitialized() : HasIntConstructor(readOwn()) {}
};

int main() {
  CallsMemberBeforeBaseIsInitialized d;
  return d.value;
}
