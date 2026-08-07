#include <assert.h>

int dispatch(int kind, int v) {
  int acc = 0;
  int scaled = 0;

  if (v < 0) {
    v = -v;
    goto negative_entry;
  }

  switch (kind) {
  case 1:
    acc = v + 1;
    break;
  case 2:
    scaled = v * 2;
  negative_entry:
    acc = scaled + v;
    break;
  default:
    acc = 999;
    break;
  }
  return acc;
}

int step(const char *p) {
  int op = 0;
  int acc = 0;

  if (*p == '!') {
    p++;
    goto forced;
  }

  switch (op = *p++) {
  case 'a':
    acc = 1;
    break;
  case 'b':
    acc = 2;
  forced:
    acc += 10;
    break;
  default:
    acc = 100;
    break;
  }
  return acc + op;
}

int main(void) {
  assert(step("a") == 1 + 'a');
  assert(step("b") == 12 + 'b');
  assert(step("z") == 100 + 'z');
  assert(step("!x") == 10);

  assert(dispatch(1, 5) == 6);
  assert(dispatch(2, 5) == 15);
  assert(dispatch(7, 5) == 999);
  assert(dispatch(7, -5) == 5);
  assert(dispatch(1, -3) == 3);
  return 0;
}
