// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [dcl.init.general]/6.1: zero-initializing a scalar converts the
// integer literal 0 to its type; for a pointer to member that is the null
// member pointer value.
namespace p6_1_zero_initialized_pointer_to_member {
struct HasTwoInts {
  int first;
  int second;
};

struct HoldsSelector {
  int HasTwoInts::*selector;
};

static int HasTwoInts::*static_pm;

void test() {
  assert(static_pm == nullptr);
  assert(static_pm != &HasTwoInts::first);
  int HasTwoInts::*value_initialized{};
  assert(value_initialized == nullptr);
  assert(value_initialized != &HasTwoInts::first);
  HoldsSelector holder{};
  assert(holder.selector == nullptr);
  assert(holder.selector != &HasTwoInts::first);
}
} // namespace p6_1_zero_initialized_pointer_to_member

// N4950 [dcl.init.general]/7.1, 7.2: default-initializing a class object calls
// its default constructor; default-initializing an array default-initializes
// each element.
namespace p7_default_initialization_calls_default_constructor {
int constructed = 0;

struct Counted {
  int id;
  Counted() : id(++constructed) {}
};

void test() {
  Counted one;
  assert(constructed == 1 && one.id == 1);
  Counted many[3];
  assert(constructed == 4);
  assert(many[0].id == 2 && many[1].id == 3 && many[2].id == 4);
}
} // namespace p7_default_initialization_calls_default_constructor

// N4950 [dcl.init.general]/9.1.1: value-initializing a class with a
// user-provided default constructor only calls that constructor.
namespace p9_1_1_value_initialization_calls_user_provided_constructor {
int constructed = 0;

struct Seven {
  int value;
  Seven() : value(7) { ++constructed; }
};

void test() {
  Seven braces{};
  assert(braces.value == 7 && constructed == 1);
  assert(Seven().value == 7 && constructed == 2);
  Seven *heap = new Seven();
  assert(heap->value == 7 && constructed == 3);
  delete heap;
}
} // namespace p9_1_1_value_initialization_calls_user_provided_constructor

// N4950 [dcl.init.general]/9.1.2: value-initializing a class without a
// user-provided default constructor zero-initializes the object, and then
// default-initializes it if its default constructor is non-trivial.
namespace p9_1_2_value_initialization_zeroes_then_default_initializes {
int constructed = 0;

struct Pair {
  int first;
  int second;
};

struct Member {
  int id;
  Member() : id(++constructed) {}
};

struct Holder {
  int number;
  Member member;
  int *pointer;
};

void test() {
  Pair braces{};
  assert(braces.first == 0 && braces.second == 0);
  assert(Pair().first == 0 && Pair().second == 0);

  // The storage of the second object held other values before.
  Pair *dirty = new Pair;
  dirty->first = 0x5a5a;
  dirty->second = 0x5a5a;
  delete dirty;
  Pair *heap = new Pair();
  assert(heap->first == 0 && heap->second == 0);
  delete heap;

  Holder holder{};
  assert(holder.number == 0 && holder.pointer == nullptr);
  assert(constructed == 1 && holder.member.id == 1);
  Holder *heap_holder = new Holder();
  assert(heap_holder->number == 0 && heap_holder->pointer == nullptr);
  assert(constructed == 2 && heap_holder->member.id == 2);
  delete heap_holder;
}
} // namespace p9_1_2_value_initialization_zeroes_then_default_initializes

// N4950 [dcl.init.general]/9.2, 9.3: value-initializing an array
// value-initializes each element; value-initializing a scalar
// zero-initializes it.
namespace p9_2_value_initialized_array_and_scalar {
struct Pair {
  int first;
  int second;
};

void test() {
  Pair pairs[2]{};
  assert(pairs[0].first == 0 && pairs[1].second == 0);
  int numbers[3]{};
  assert(numbers[0] == 0 && numbers[2] == 0);
  int number{};
  int *pointer{};
  assert(number == 0 && pointer == nullptr);
}
} // namespace p9_2_value_initialized_array_and_scalar

int main() {
  p7_default_initialization_calls_default_constructor::test();
  p9_1_1_value_initialization_calls_user_provided_constructor::test();
  p9_1_2_value_initialization_zeroes_then_default_initializes::test();
  p9_2_value_initialized_array_and_scalar::test();
  p6_1_zero_initialized_pointer_to_member::test();
  return 0;
}
