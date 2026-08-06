#include <assert.h>

static int fails = 0;

static int fail_mark(void) {
  fails++;
  return -1;
}

static int helper(int mode, int v) {
  int r = 0;
  if (mode == 1) {
    if (v < 0)
      goto bad_input;
    r = v * 2;
  } else if (mode == 2) {
    if (v == 0)
      goto bad_input;
    r = 100 / v;
  } else
  bad_input:
    r = fail_mark();
  return r;
}

int main(void) {
  assert(helper(1, 4) == 8);
  assert(helper(1, -1) == -1);
  assert(helper(2, 5) == 20);
  assert(helper(2, 0) == -1);
  assert(helper(7, 3) == -1);
  assert(fails == 3);
  return 0;
}
