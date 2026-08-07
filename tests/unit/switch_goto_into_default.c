#include <assert.h>

static int direct_label(int x, int y) {
  switch (x) {
  case 1:
    if (y)
      goto other;
    return 10;
  case 2:
    return 30;
  default:
  other:
    return 20;
  }
}

static int braced_label(int x, int y) {
  int r = 0;
  switch (x) {
  case 1:
    if (y)
      goto other;
    r = 10;
    break;
  case 2:
    r = 30;
    break;
  default: {
  other:
    r = 20;
    break;
  }
  }
  return r;
}

int main(void) {
  assert(direct_label(1, 0) == 10);
  assert(direct_label(1, 1) == 20);
  assert(direct_label(2, 0) == 30);
  assert(direct_label(5, 0) == 20);
  assert(direct_label(0, 1) == 20);

  assert(braced_label(1, 0) == 10);
  assert(braced_label(1, 1) == 20);
  assert(braced_label(2, 0) == 30);
  assert(braced_label(5, 0) == 20);
  assert(braced_label(0, 1) == 20);

  return 0;
}
