#include <cassert>

int global = 0;

struct S {
  ~S() { global++; }
};

  ~Inner() {
    if (target != nullptr) {
      total += *target;
      target = nullptr;
    }
  }
};

struct Outer {
  Middle m;
};

struct ArrayMember {
  S items[3];
};

struct EmptyBody {
  S s;
  ~EmptyBody() {}
};

template <typename T> struct Templated {
  T v;
  ~Templated() { global += sizeof(T); }
};

OutOfLine::~OutOfLine() { total += step; }

int main() {
  {
    S s{};
  }
  assert(global == 1);

  {
    S s{};
  }
  assert(global == 2);

  {
    Defaulted d{};
  }
  assert(global == 3);

  {
    Outer o{};
  }
  assert(global == 4);

  {
    ArrayMember am{};
  }
  assert(global == 7);

  {
    EmptyBody e{};
  }
  assert(global == 8);

  {
    Templated<char> tc{};
    Templated<int> ti{};
  }
  assert(global == 13);

  {
    Copied a{5};
    Copied b = a;
    assert(b.v == 5);
  }
  assert(global == 15);

  return 0;
}
