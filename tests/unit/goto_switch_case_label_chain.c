#include <assert.h>

int pick(int op, int x) {
  int r = 0;
  switch (op) {
  case 1:
    if (x == 0) {
      r = 5;
      break;
    }
    goto shared;
  case 2:
  shared: {
    int t = x * 3;
    r = t + 1;
    break;
  }
  default:
    r = -1;
    break;
  }
  return r;
}

int main(void) {
  assert(pick(1, 0) == 5);
  assert(pick(1, 4) == 13);
  assert(pick(2, 2) == 7);
  assert(pick(0, 9) == -1);
  return 0;
}
