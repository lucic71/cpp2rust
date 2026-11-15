// nondet-result
// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.derived.general]/5: the order in which the base class
// subobjects are allocated in the most derived object is unspecified. The
// exit code reports which of the two base subobjects has the lower address.

struct FirstBase {
  int first = 1;
};

struct SecondBase {
  int second = 2;
};

struct HasTwoBases : FirstBase, SecondBase {};

int main() {
  HasTwoBases d;
  FirstBase *first = &d;
  SecondBase *second = &d;
  void *firstAddress = first;
  void *secondAddress = second;
  return firstAddress < secondAddress ? 1 : 2;
}
