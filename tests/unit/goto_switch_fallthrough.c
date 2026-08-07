#include <assert.h>

static int sm(int n) {
  int ret = 0;
  switch (n) {
  case 0:
    ret += 1;
    /* fallthrough */
  case 1:
    ret += 10;
    goto out;
  default:
    ret += 100;
    break;
  }
  ret += 1000;
out:
  return ret;
}

static int scan(const char *p) {
  int c = 0;
  int ret = 0;
  switch (c = *p++) {
  case 'a':
    ret = 1;
    /* fallthrough */
  case 'b':
    ret += 10;
    goto out;
  default:
    ret = 100;
    break;
  }
  ret += 1000;
out:
  return ret + c;
}

int main(void) {
  assert(sm(0) == 11);
  assert(sm(1) == 10);
  assert(sm(9) == 1100);

  assert(scan("a") == 11 + 'a');
  assert(scan("b") == 10 + 'b');
  assert(scan("z") == 1100 + 'z');
  return 0;
}
