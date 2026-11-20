// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <cstdio>

struct Logged {
  int id;
  Logged(int id) : id(id) { printf("member %d\n", id); }
};

// N4950 [class.base.init]/6: a delegating constructor runs its target
// constructor, and then its own body.
namespace p6_delegating_constructor_runs_target_first {
struct Delegating {
  int value;
  Delegating(int value) : value(value) { printf("target %d\n", value); }
  Delegating() : Delegating(42) {
    printf("delegating %d\n", value);
    value += 1;
  }
};

void test() {
  Delegating object;
  assert(object.value == 43);
}
} // namespace p6_delegating_constructor_runs_target_first

// N4950 [class.base.init]/9.1: a member without a mem-initializer is
// initialized from its default member initializer; a mem-initializer
// replaces the default member initializer, which is then not evaluated.
namespace p9_1_default_member_initializer {
int evaluated = 0;

int next() { return ++evaluated; }

struct WithDefaults {
  int fixed = 5;
  int counted = next();
  WithDefaults() {}
  WithDefaults(int counted) : counted(counted) {}
};

void test() {
  WithDefaults defaults;
  assert(defaults.fixed == 5 && defaults.counted == 1 && evaluated == 1);
  WithDefaults given(9);
  assert(given.fixed == 5 && given.counted == 9 && evaluated == 1);
}
} // namespace p9_1_default_member_initializer

// N4950 [class.base.init]/9.3: a member of class type without a
// mem-initializer or default member initializer is default-initialized.
namespace p9_3_member_is_default_initialized {
int constructed = 0;

struct Member {
  int id;
  Member() : id(++constructed) {}
};

struct Holder {
  Member first;
  Member second;
  int number;
  Holder() : number(3) {}
};

void test() {
  Holder holder;
  assert(constructed == 2 && holder.first.id == 1 && holder.second.id == 2);
  assert(holder.number == 3);
}
} // namespace p9_3_member_is_default_initialized

// N4950 [class.base.init]/13.3, 13.4: members are initialized in the order
// they are declared, regardless of the order of the mem-initializers, and
// the constructor body runs last.
namespace p13_members_initialized_in_declaration_order {
struct Ordered {
  Logged first;
  Logged second;
  int sum;
  Ordered() : sum(first.id + second.id), second(2), first(1) {
    printf("body %d\n", sum);
  }
};

void test() {
  Ordered ordered;
  assert(ordered.sum == 3);
}
} // namespace p13_members_initialized_in_declaration_order

int main() {
  p6_delegating_constructor_runs_target_first::test();
  p9_1_default_member_initializer::test();
  p9_3_member_is_default_initialized::test();
  p13_members_initialized_in_declaration_order::test();
  return 0;
}
