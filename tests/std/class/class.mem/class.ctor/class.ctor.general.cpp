// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <cstdio>
#include <utility>

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

namespace local_variable {
void test() {
  Tracker a(1);
  assert(a.self == &a && a.value == 1);
}
} // namespace local_variable

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

namespace default_constructor {
void test() {
  Tracker a;
  assert(a.self == &a && a.value == -1);
  Tracker elements[2];
  assert(elements[1].self == &elements[1] && elements[1].value == -1);
}
} // namespace default_constructor

namespace delegating_constructor {
void test() {
  Tracker a(1, 2);
  assert(a.self == &a && a.value == 3);
}
} // namespace delegating_constructor

namespace copy_constructor {
void test() {
  Tracker a(1);
  Tracker b(a);
  assert(b.self == &b && b.value == 1 && a.self == &a && a.value == 1);
}
} // namespace copy_constructor

namespace move_constructor {
void test() {
  Tracker a(1);
  Tracker b(std::move(a));
  assert(b.self == &b && b.value == 1 && a.self == &a && a.value == -2);
}
} // namespace move_constructor

// N4950 [class.default.ctor]/4, [class.copy.ctor]/14: an implicitly-defined
// constructor initializes each member with the corresponding constructor.
namespace implicit_constructors {
struct Pair {
  Tracker first;
  Tracker second;
};

void test() {
  Pair a;
  assert(a.first.self == &a.first && a.second.self == &a.second);
  a.second.value = 5;
  Pair b(a);
  assert(b.first.self == &b.first && b.second.self == &b.second);
  assert(b.second.value == 5);
  Pair c(std::move(a));
  assert(c.second.self == &c.second && c.second.value == 5);
  assert(a.second.value == -2);
}
} // namespace implicit_constructors

namespace new_expression {
void test() {
  Tracker *p = new Tracker(4);
  assert(p->self == p && p->value == 4);
  delete p;
}
} // namespace new_expression

namespace member {
struct Holder {
  int before;
  Tracker tracker;
  Holder(int value) : before(value), tracker(before + 1) {}
};

void test() {
  Holder h(5);
  assert(h.tracker.self == &h.tracker && h.tracker.value == 6);
}
} // namespace member

namespace array_element {
void test() {
  Tracker elements[2] = {Tracker(6), Tracker(7)};
  assert(elements[0].self == &elements[0] && elements[0].value == 6);
  assert(elements[1].self == &elements[1] && elements[1].value == 7);
}
} // namespace array_element

// N4950 [dcl.init.general]/16.6.1: a prvalue of the class of the destination
// initializes the destination object, with no copy or move.
namespace function_argument {
bool is_self(Tracker tracker) { return tracker.self == &tracker; }

void test() { assert(is_self(Tracker(8))); }
} // namespace function_argument

namespace returned_object {
Tracker make(int value) { return Tracker(value); }

struct Aggregate {
  Tracker tracker;
};

void test() {
  Tracker r = make(9);
  assert(r.self == &r && r.value == 9);
  Aggregate aggregate{make(10)};
  assert(aggregate.tracker.self == &aggregate.tracker);
}
} // namespace returned_object

namespace temporary {
void test() {
  assert(Tracker(11).is_self());
  const Tracker &bound = Tracker(12);
  assert(bound.self == &bound && bound.value == 12);
}
} // namespace temporary

namespace conditional_operand {
void test() {
  for (int i = 0; i < 2; ++i) {
    Tracker chosen = i == 0 ? Tracker(13) : Tracker(14);
    assert(chosen.self == &chosen && chosen.value == 13 + i);
  }
}
} // namespace conditional_operand

// N4950 [class.dtor]/9: each object is constructed once, where it lives, and
// destroyed once, in the reverse order of construction.
namespace lifetime {
struct Logged {
  int id;
  Logged(int id) : id(id) { printf("ctor %d\n", id); }
  Logged(const Logged &other) : id(other.id + 100) { printf("copy %d\n", id); }
  ~Logged() { printf("dtor %d\n", id); }
};

Logged make(int id) { return Logged(id); }

void test() {
  Logged local(1);
  Logged copy(local);
  Logged returned = make(3);
  printf("end\n");
}
} // namespace lifetime

int main() {
  lifetime::test();
  local_variable::test();
  static_local_variable::test();
  global_variable::test();
  default_constructor::test();
  delegating_constructor::test();
  copy_constructor::test();
  move_constructor::test();
  implicit_constructors::test();
  new_expression::test();
  member::test();
  array_element::test();
  function_argument::test();
  returned_object::test();
  temporary::test();
  conditional_operand::test();
  return 0;
}
