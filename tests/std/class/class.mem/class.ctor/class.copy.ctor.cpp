// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>
#include <cstdio>
#include <initializer_list>
#include <utility>
#include <vector>

// N4950 [class.copy.ctor]/11: a copy/move constructor that is not
// user-provided is trivial.
namespace p11_defaulted_move_constructor_is_trivial {
struct MoveOnly {
  int value;
  MoveOnly(int value) : value(value) {}
  MoveOnly(const MoveOnly &) = delete;
  MoveOnly(MoveOnly &&) = default;
};

MoveOnly pass(MoveOnly object) { return object; }

void test() {
  MoveOnly result = pass(MoveOnly(7));
  assert(result.value == 7);
}
} // namespace p11_defaulted_move_constructor_is_trivial

// N4950 [class.copy.ctor]/14: the implicitly-defined move constructor
// performs a memberwise move, so a trivial move leaves the source unchanged.
namespace p14_trivial_move_keeps_source {
int read_by_value(std::vector<int>::iterator it) { return *it; }

void test() {
  std::initializer_list<int> source = {1, 2, 3};
  std::initializer_list<int> moved = std::move(source);
  assert(moved.size() == 3);
  assert(source.size() == 3);
  std::vector<int> values{7, 8};
  std::vector<int>::iterator it = values.begin();
  assert(read_by_value(std::move(it)) == 7);
  assert(*it == 7);
  std::vector<int>::iterator moved_it = std::move(it);
  assert(*moved_it == 7);
  assert(*it == 7);
}
} // namespace p14_trivial_move_keeps_source

// N4950 [class.copy.ctor]/14: the implicitly-defined copy/move constructor
// copies/moves each member, in the order of their declaration, with the
// constructor of the member, and each element of an array member.
namespace p14_memberwise_copy_and_move_in_declaration_order {
struct Logged {
  int id;
  Logged(int id) : id(id) {}
  Logged(const Logged &other) : id(other.id) { printf("copy %d\n", id); }
  Logged(Logged &&other) : id(other.id) {
    printf("move %d\n", id);
    other.id = -1;
  }
};

struct Holder {
  Logged first;
  int number;
  Logged elements[2];
  Logged last;
};

void test() {
  Holder source{Logged(1), 5, {Logged(2), Logged(3)}, Logged(4)};
  Holder copy(source);
  assert(copy.first.id == 1 && copy.number == 5 && copy.last.id == 4);
  assert(copy.elements[0].id == 2 && copy.elements[1].id == 3);
  assert(source.first.id == 1 && source.last.id == 4);
  Holder moved(std::move(source));
  assert(moved.first.id == 1 && moved.number == 5 && moved.last.id == 4);
  assert(moved.elements[0].id == 2 && moved.elements[1].id == 3);
  assert(source.first.id == -1 && source.elements[1].id == -1);
  assert(source.last.id == -1 && source.number == 5);
}
} // namespace p14_memberwise_copy_and_move_in_declaration_order

int main() {
  p14_memberwise_copy_and_move_in_declaration_order::test();
  p11_defaulted_move_constructor_is_trivial::test();
  p14_trivial_move_keeps_source::test();
  return 0;
}
