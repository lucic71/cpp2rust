#include <assert.h>

int compute(int op, int a, int b) {
  int r = 0;
  if (a > 0) {
  int_path:
    r = a + b;
    if (op)
      goto fp_path;
  } else {
    if (b > 0)
      goto int_path;
  fp_path:
    r = a * b;
  }
  return r;
}

int main(void) {
  assert(compute(0, 5, 3) == 8);
  assert(compute(1, 5, 3) == 15);
  assert(compute(0, -2, 4) == 2);
  assert(compute(0, -2, -4) == 8);
  assert(compute(1, -2, -4) == 8);
  return 0;
}
