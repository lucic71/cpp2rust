#include <cassert>

static int total = 0;

struct Inner {
  int *target = nullptr;

  ~Inner() {
    if (target != nullptr) {
      total += *target;
      target = nullptr;
    }
  }
};

struct Outer {
  Inner inner;
};

struct OutOfLine {
  int step = 4;

  ~OutOfLine();
};

OutOfLine::~OutOfLine() { total += step; }

int main() {
  int value = 40;
  {
    Outer o;
    o.inner.target = &value;
  }
  assert(total == 40);

  { OutOfLine t; }
  assert(total == 44);

  return 0;
}
