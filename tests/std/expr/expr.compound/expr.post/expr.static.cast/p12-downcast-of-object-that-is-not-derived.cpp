// panic-ub: refcount
// nondet-result: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.static.cast]/12: if the pointer to B does not point to a base
// class subobject of an object of type D, converting it to a pointer to D is
// undefined.

struct Base {
  int base = 1;
};

struct Derived : Base {
  int derived = 2;
};

int main() {
  Base onlyBase;
  Base *base = &onlyBase;
  Derived *derived = static_cast<Derived *>(base);
  return derived->derived;
}
