// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.mi]/4 (figure 6): a class can have both virtual and non-virtual
// base classes of a given type. A HasSharedAndOwnBase object has two
// RepeatedBase subobjects: the virtual one shared by FirstVirtualPath and
// SecondVirtualPath, and the one that belongs to NonVirtualPath.
#include <cassert>

static int repeated_base_ctors;

struct RepeatedBase {
  int value;
  RepeatedBase() : value(0) { ++repeated_base_ctors; }
};

struct FirstVirtualPath : virtual RepeatedBase {};
struct SecondVirtualPath : virtual RepeatedBase {};
struct NonVirtualPath : RepeatedBase {};

struct HasSharedAndOwnBase : FirstVirtualPath,
                             SecondVirtualPath,
                             NonVirtualPath {};

int main() {
  HasSharedAndOwnBase d;
  assert(repeated_base_ctors == 2);

  FirstVirtualPath *firstVirtual = &d;
  SecondVirtualPath *secondVirtual = &d;
  NonVirtualPath *nonVirtual = &d;
  RepeatedBase *throughFirstVirtual = firstVirtual;
  RepeatedBase *throughSecondVirtual = secondVirtual;
  RepeatedBase *throughNonVirtual = nonVirtual;
  assert(throughFirstVirtual == throughSecondVirtual);
  assert(throughFirstVirtual != throughNonVirtual);

  d.FirstVirtualPath::value = 1;
  d.NonVirtualPath::value = 2;
  assert(d.SecondVirtualPath::value == 1);
  assert(throughFirstVirtual->value == 1);
  assert(throughSecondVirtual->value == 1);
  assert(throughNonVirtual->value == 2);
  throughSecondVirtual->value = 3;
  assert(d.FirstVirtualPath::value == 3);
  assert(d.NonVirtualPath::value == 2);
  return 0;
}
