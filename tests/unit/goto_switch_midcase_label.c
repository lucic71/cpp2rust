#include <assert.h>

int classify(int kind, int x) {
  int len = 0;
  int width = 0;
  switch (kind) {
  case 0:
    width = x * 2;
    goto finish_width;
  case 1:
    len = x + 1;
    if (len > 10)
      len = 10;
  finish_width:
    width += len;
    width++;
    break;
  case 2:
    len = 50;
    goto finish_width;
  default:
    width = -1;
    break;
  }
  return width;
}

int main(void) {
  assert(classify(0, 4) == 9);
  assert(classify(1, 2) == 4);
  assert(classify(1, 42) == 11);
  assert(classify(2, 0) == 51);
  assert(classify(7, 0) == -1);
  return 0;
}
