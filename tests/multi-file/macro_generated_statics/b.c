#include "helpers.h"

int combine(int x) {
  return scaleA(x) + scaleB(x) + shiftA(x) + shiftB(x) + (int)pmax_long(x, 0);
}
