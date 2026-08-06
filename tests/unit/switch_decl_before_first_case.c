#include <assert.h>

static int reduce(int rule, int v) {
  int acc = 0;
  switch (rule) {
    int tmp;
    long wide;
  case 0:
    tmp = v * 2;
    acc = tmp + 1;
    break;
  case 1:
    wide = (long)v + 10;
    acc = (int)(wide * 2);
    break;
  case 2:
    tmp = v - 1;
    wide = tmp;
    acc = (int)wide * 3;
    break;
  default:
    acc = -1;
    break;
  }
  return acc;
}

int main(void) {
  assert(reduce(0, 5) == 11);
  assert(reduce(1, 5) == 30);
  assert(reduce(2, 5) == 12);
  assert(reduce(9, 5) == -1);
  return 0;
}
