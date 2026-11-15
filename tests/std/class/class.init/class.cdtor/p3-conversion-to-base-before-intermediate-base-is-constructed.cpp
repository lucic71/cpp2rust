// panic-ub: refcount
// nondet-result: unsafe
// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [class.cdtor]/3: converting a pointer to a class into a pointer to a
// base requires that the construction of every base that derives from that
// base has started; otherwise the conversion is undefined. The argument
// converts this to HasValue* before the construction of the intermediate base
// StoresBasePointer has started.

struct HasValue {
  int value = 3;
};

struct StoresBasePointer : HasValue {
  HasValue *stored;
  StoresBasePointer(HasValue *base) : stored(base) {}
};

struct ConvertsThisTooEarly : StoresBasePointer {
  ConvertsThisTooEarly() : StoresBasePointer(this) {}
};

int main() {
  ConvertsThisTooEarly d;
  return d.stored->value;
}
