#include <assert.h>

int main(void) {
  int a = +5;
  double d = +0.5;
  int b = -3;
  assert(+a == 5);
  assert(+b == -3);
  assert(a + +b == 2);
  assert(+d * 4.0 == 2.0);
  assert((+a > 0 ? +1 : -1) == 1);
  return 0;
}
