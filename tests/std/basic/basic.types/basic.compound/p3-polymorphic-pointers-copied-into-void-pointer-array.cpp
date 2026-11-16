// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.compound]/3: the value representation of pointer types is
// implementation-defined but they have to be compatible with void*.
#include <cassert>
#include <cstring>

static int first_calls, second_calls, third_calls;

struct DeclaresVirtual {
  virtual void f() = 0;
};

struct FirstOverrider : DeclaresVirtual {
  void f() override { ++first_calls; }
};

struct SecondOverrider : DeclaresVirtual {
  void f() override { ++second_calls; }
};

struct ThirdOverrider : DeclaresVirtual {
  void f() override { ++third_calls; }
};

int main() {
  FirstOverrider first;
  SecondOverrider second;
  ThirdOverrider third;
  DeclaresVirtual *polymorphicPointers[3] = {&first, &second, &third};

  void *voidPointers[3] = {};
  std::memcpy(voidPointers, polymorphicPointers, sizeof(voidPointers));

  assert(voidPointers[0] == &first);
  assert(voidPointers[1] == &second);
  assert(voidPointers[2] == &third);

  static_cast<DeclaresVirtual *>(voidPointers[0])->f();
  assert(first_calls == 1 && second_calls == 0 && third_calls == 0);
  static_cast<DeclaresVirtual *>(voidPointers[1])->f();
  assert(first_calls == 1 && second_calls == 1 && third_calls == 0);
  static_cast<DeclaresVirtual *>(voidPointers[2])->f();
  assert(first_calls == 1 && second_calls == 1 && third_calls == 1);
  return 0;
}
