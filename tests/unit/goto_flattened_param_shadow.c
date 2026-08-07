#include <assert.h>

static int dispatch(int op, int flags) {
  int r = 0;

  if (op == 1) {
  from_op:
    {
      int flags;
      flags = 7;
      r += flags;
    }
  } else {
    if (op == 2)
      goto from_op;
    r += 100;
  }

  if (flags & 4)
    r += 1000;
  return r;
}

int main(void) {
  assert(dispatch(1, 4) == 1007);
  assert(dispatch(0, 4) == 1100);
  assert(dispatch(2, 4) == 1007);
  assert(dispatch(1, 0) == 7);
  return 0;
}
