#include <assert.h>

static int step(int mode, int v) {
  static const int base[] = {100, 200};
  static int calls = 0;
  int r = 0;

  calls++;
  if (v > 0) {
  from_positive:
    r = base[0] + v;
    if (mode == 1)
      goto from_negative;
  } else {
    if (mode == 2)
      goto from_positive;
  from_negative:
    r = base[1] - v;
  }
  return r * 10 + calls;
}

int main(void) {
  assert(step(0, 5) == 1051);
  assert(step(1, 5) == 1952);
  assert(step(0, -2) == 2023);
  assert(step(2, -2) == 984);
  return 0;
}
