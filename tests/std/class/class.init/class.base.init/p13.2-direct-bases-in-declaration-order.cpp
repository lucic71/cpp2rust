// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.base.init]/13.2 and /7: direct base classes are initialized in
// declaration order as they appear in the base-specifier-list, regardless of
// the order of the mem-initializers. The argument of each mem-initializer is
// evaluated as part of that initialization.
#include <cassert>

static int sequence;

static int stampArgument(int &stamp, int value) {
  stamp = ++sequence;
  return value;
}

struct FirstBase {
  int value, stamp;
  FirstBase(int v) : value(v), stamp(++sequence) {}
};

struct SecondBase {
  int value, stamp;
  SecondBase(int v) : value(v), stamp(++sequence) {}
};

struct InitializesBasesOutOfOrder : FirstBase, SecondBase {
  int firstArgumentStamp, secondArgumentStamp;
  InitializesBasesOutOfOrder(int a)
      : SecondBase(stampArgument(secondArgumentStamp, a + 1)),
        FirstBase(stampArgument(firstArgumentStamp, a + 2)) {}
};

int main() {
  InitializesBasesOutOfOrder d(10);
  assert(d.FirstBase::value == 12);
  assert(d.SecondBase::value == 11);
  assert(d.firstArgumentStamp == 1);
  assert(d.FirstBase::stamp == 2);
  assert(d.secondArgumentStamp == 3);
  assert(d.SecondBase::stamp == 4);
  assert(sequence == 4);
  return 0;
}
