// translation-fail: refcount
// no-compile: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.virtual]/9: when the overriding function is called as the final
// overrider of the overridden function, its result is converted to the type
// returned by the statically chosen overridden function. Result is not the
// first base of DerivedResult, so the conversion has to yield the Result
// subobject, not the DerivedResult object itself.
#include <cassert>

static int base_calls, derived_calls;

struct OtherBase {
  int other = 1;
};

struct Result {
  int value = 2;
};

struct DerivedResult : OtherBase, Result {
  int extra = 3;
};

struct ReturnsResult {
  Result result;
  virtual Result *get() {
    ++base_calls;
    return &result;
  }
};

struct ReturnsDerivedResult : ReturnsResult {
  DerivedResult derivedResult;
  DerivedResult *get() override {
    ++derived_calls;
    return &derivedResult;
  }
};

int main() {
  ReturnsDerivedResult f;
  ReturnsResult *base = &f;

  Result *converted = base->get();
  assert(base_calls == 0 && derived_calls == 1);
  Result *expected = &f.derivedResult;
  assert(converted == expected);
  assert(converted->value == 2);
  converted->value = 9;
  assert(f.derivedResult.value == 9);
  assert(f.derivedResult.other == 1);
  assert(f.derivedResult.extra == 3);
  assert(f.result.value == 2);

  DerivedResult *notConverted = f.get();
  assert(base_calls == 0 && derived_calls == 2);
  assert(notConverted == &f.derivedResult);
  notConverted->extra = 7;
  assert(f.derivedResult.extra == 7);
  return 0;
}
