#include <assert.h>

static int run(int x) {
  int steps = 0;
  if (x < 0) {
    goto error;
  }
  steps = 1;
  if (x == 0) {
    goto done;
  }
  steps = 2;
error:
done:
  steps += 10;
  return steps;
}

int main(void) {
  assert(run(-1) == 10);
  assert(run(0) == 11);
  assert(run(5) == 12);
  return 0;
}
