// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.base.init]/18: a mem-initializer followed by an ellipsis is a
// pack expansion that initializes the base classes specified by a pack
// expansion in the base-specifier-list. Each base is copy-initialized from its
// own argument, in base order.
#include <cassert>

static int sequence;

struct FirstMixin {
  int value, copyStamp = 0;
  FirstMixin(int v) : value(v) {}
  FirstMixin(const FirstMixin &o) : value(o.value), copyStamp(++sequence) {}
};

struct SecondMixin {
  int value, copyStamp = 0;
  SecondMixin(int v) : value(v) {}
  SecondMixin(const SecondMixin &o) : value(o.value), copyStamp(++sequence) {}
};

template <class... Mixins> struct CopiesMixinsIntoBases : Mixins... {
  CopiesMixinsIntoBases(const Mixins &...mixins) : Mixins(mixins)... {}
};

int main() {
  FirstMixin first(1);
  SecondMixin second(2);
  CopiesMixinsIntoBases<FirstMixin, SecondMixin> both(first, second);
  assert(both.FirstMixin::value == 1);
  assert(both.SecondMixin::value == 2);
  assert(both.FirstMixin::copyStamp == 1);
  assert(both.SecondMixin::copyStamp == 2);
  assert(first.copyStamp == 0 && second.copyStamp == 0);

  CopiesMixinsIntoBases<SecondMixin> one(second);
  assert(one.value == 2);
  assert(one.copyStamp == 3);

  CopiesMixinsIntoBases<> none;
  (void)none;
  assert(sequence == 3);
  return 0;
}
