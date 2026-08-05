#include <assert.h>

int route(int op, int v) {
  int out = 0;
  switch (op) {
  case 1: {
    int base = v * 10;
    if (v > 3) {
      out = base + 1;
      goto tail;
    }
    v = base;
    /* fall through */
  case 2:
    base = v + 7;
    out = base * 2;
  tail:
    out += 3;
    break;
  }
  case 3:
    out = -v;
    break;
  default:
    out = 99;
    break;
  }
  return out;
}

int main(void) {
  assert(route(1, 5) == 54);
  assert(route(1, 2) == 57);
  assert(route(2, 10) == 37);
  assert(route(3, 4) == -4);
  assert(route(9, 0) == 99);
  return 0;
}
