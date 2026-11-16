// no-compile: refcount
// XFAIL: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.compound]/3: the value representation of pointer types is
// implementation-defined but they have to be compatible with void*.
#include <cassert>
#include <cstdlib>

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

  DeclaresVirtual **table =
      static_cast<DeclaresVirtual **>(std::malloc(3 * sizeof(void *)));
  table[0] = &first;
  table[1] = &second;
  table[2] = &third;

  assert(table[0] == &first);
  assert(table[1] == &second);
  assert(table[2] == &third);

  table[0]->f();
  assert(first_calls == 1 && second_calls == 0 && third_calls == 0);
  table[1]->f();
  assert(first_calls == 1 && second_calls == 1 && third_calls == 0);
  table[2]->f();
  assert(first_calls == 1 && second_calls == 1 && third_calls == 1);

  std::free(table);
  return 0;
}
