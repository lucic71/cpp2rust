// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [basic.life]/1.5: the lifetime of an object ends when the storage it
// occupies is reused by an object that is not nested within it.
#include <cassert>
#include <new>

static int first_destructions, second_destructions;

struct DeclaresVirtual {
  virtual int id() = 0;
  virtual ~DeclaresVirtual() {}
};

struct FirstOverrider : DeclaresVirtual {
  int id() override { return 1; }
  ~FirstOverrider() override { ++first_destructions; }
};

struct SecondOverrider : DeclaresVirtual {
  int id() override { return 2; }
  ~SecondOverrider() override { ++second_destructions; }
};

int main() {
  alignas(DeclaresVirtual) unsigned char storage[64];

  DeclaresVirtual *object = new (storage) FirstOverrider;
  assert(object->id() == 1);
  object->~DeclaresVirtual();
  assert(first_destructions == 1 && second_destructions == 0);

  object = new (storage) SecondOverrider;
  assert(object->id() == 2);
  object->~DeclaresVirtual();
  assert(first_destructions == 1 && second_destructions == 1);
  return 0;
}
