// XFAIL: unsafe
// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [class.ctor.general]: a constructor initializes the object that this
// points to, which is the object the declaration or expression creates. The
// copy constructor keeps the class from being passed or returned in a
// temporary ([class.temporary]/3).
struct Tracker {
  Tracker *self;
  int value;
  Tracker() : self(this), value(-1) {}
  Tracker(int value) : self(this), value(value) {}
  Tracker(int a, int b) : Tracker(a + b) {}
  Tracker(const Tracker &other) : self(this), value(other.value) {}
  Tracker(Tracker &&other) : self(this), value(other.value) {
    other.value = -2;
  }
  bool is_self() const { return self == this; }
};

namespace static_local_variable {
Tracker *get() {
  static Tracker once(2);
  return &once;
}

void test() {
  Tracker *p = get();
  assert(p->self == p && p->value == 2);
  assert(get() == p);
}
} // namespace static_local_variable

namespace global_variable {
Tracker global(3);

void test() { assert(global.self == &global && global.value == 3); }
} // namespace global_variable

int main() {
  static_local_variable::test();
  global_variable::test();
  return 0;
}
