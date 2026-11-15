// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.base.init]/9.3: a base class subobject not designated by a
// mem-initializer-id is default-initialized, whether the constructor has no
// ctor-initializer, one that names only another base, or is implicit.
#include <cassert>

struct CountsConstructors {
  static inline int defaultCtors = 0;
  static inline int intCtors = 0;

  int value;

  CountsConstructors() : value(-1) { ++defaultCtors; }
  CountsConstructors(int v) : value(v) { ++intCtors; }
};

struct OtherBase {
  int other;
  OtherBase(int v) : other(v) {}
};

struct HasNoCtorInitializer : CountsConstructors {
  HasNoCtorInitializer() {}
  HasNoCtorInitializer(int) {}
};

struct InitializesOnlyOtherBase : CountsConstructors, OtherBase {
  InitializesOnlyOtherBase() : OtherBase(5) {}
};

struct HasImplicitConstructor : CountsConstructors {};

struct InitializesBase : CountsConstructors {
  InitializesBase() : CountsConstructors(7) {}
};

int main() {
  HasNoCtorInitializer a;
  assert(CountsConstructors::defaultCtors == 1);
  assert(a.value == -1);

  HasNoCtorInitializer b(1);
  assert(CountsConstructors::defaultCtors == 2);
  assert(b.value == -1);

  InitializesOnlyOtherBase c;
  assert(CountsConstructors::defaultCtors == 3);
  assert(c.value == -1 && c.other == 5);

  HasImplicitConstructor d;
  assert(CountsConstructors::defaultCtors == 4);
  assert(d.value == -1);

  InitializesBase e;
  assert(CountsConstructors::defaultCtors == 4);
  assert(CountsConstructors::intCtors == 1);
  assert(e.value == 7);
  return 0;
}
