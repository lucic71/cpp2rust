#include <assert.h>

#include "helpers.h"

int combine(int x);

int main(void) {
  assert(scaleA(5) == 10);
  assert(scaleB(5) == 15);
  assert(shiftA(5) == 3);
  assert(shiftB(5) == 2);
  assert(pmin_int(3, 4) == 3);
  assert(pmax_int(3, 4) == 4);
  assert(pmin_long(3, 4) == 3);
  assert(pmax_long(3, 4) == 4);
  assert(combine(5) == 35);
  return 0;
}
