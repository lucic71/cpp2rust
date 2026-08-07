// panic-ub
#include <assert.h>

struct bits {
  unsigned b : 3;
  unsigned w : 12;
  int s : 3;
};

int main(void) {
  volatile int nine = 9, big = 0x1234, seven = 7;
  struct bits v;
  v.b = 0;
  v.w = 0;
  v.s = 0;

  v.b = nine;
  assert(v.b == 1);

  v.b = 7;
  v.b++;
  assert(v.b == 0);

  v.b = 0;
  v.b--;
  assert(v.b == 7);

  v.w = big;
  assert(v.w == 0x234);

  v.s = seven;
  assert(v.s == -1);

  v.s = 3;
  v.s++;
  assert(v.s == -4);

  v.s = -4;
  v.s--;
  assert(v.s == 3);

  return 0;
}
