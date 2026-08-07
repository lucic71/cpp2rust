#include <cassert>

int fallthrough_default(int x, int flag) {
  int r = 0;
  switch (x) {
  case 7:
    if (flag) {
      r = 100;
      break;
    }
    [[fallthrough]];
  default:
    r = 42;
    break;
  }
  return r;
}

int breakless_default(int x) {
  int r = 0;
  switch (x) {
  case 7:
    r += 1;
    [[fallthrough]];
  default:
    r += 42;
  }
  return r + 1;
}

int main() {
  assert(fallthrough_default(7, 0) == 42);
  assert(fallthrough_default(7, 1) == 100);
  assert(fallthrough_default(99, 0) == 42);
  assert(breakless_default(7) == 44);
  assert(breakless_default(99) == 43);
  return 0;
}
