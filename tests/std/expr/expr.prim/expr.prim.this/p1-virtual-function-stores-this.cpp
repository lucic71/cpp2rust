// no-compile: refcount, unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
// N4950 [expr.prim.this]/1: the keyword this names a pointer to the object for
// which the member function is invoked.
#include <cassert>

struct DeclaresVirtual {
  int value = 0;
  virtual void registerSelf() = 0;
};

static DeclaresVirtual *registered_globally;

struct Registry {
  DeclaresVirtual *entry = nullptr;
};

struct StoresThis : DeclaresVirtual {
  Registry *registry;
  StoresThis(Registry *r) : registry(r) {}
  void registerSelf() override {
    registered_globally = this;
    registry->entry = this;
  }
};

int main() {
  Registry registry;
  StoresThis d(&registry);
  DeclaresVirtual *base = &d;
  assert(registered_globally == nullptr);
  assert(registry.entry == nullptr);

  base->registerSelf();
  assert(registered_globally == base);
  assert(registry.entry == base);

  registered_globally->value = 3;
  assert(d.value == 3);
  registry.entry->value = 4;
  assert(d.value == 4);
  return 0;
}
