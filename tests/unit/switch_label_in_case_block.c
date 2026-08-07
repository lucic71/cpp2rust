#include <assert.h>
#include <string.h>

static int f(int op, int v) {
  int r = 0;

  switch (op) {
  case 1: {
    int a = v * 4;
    if (v)
      goto l1;
    break;
  l1:
    r = a;
  } break;
  case 2: {
    const char *a = "abcd";
    if (v)
      goto l2;
    break;
  l2:
    r = (int)strlen(a);
  } break;
  }
  return r;
}

int main(void) {
  assert(f(1, 3) == 12);
  assert(f(2, 1) == 4);
  assert(f(1, 0) == 0);
  return 0;
}
