#include <assert.h>

static int run(void) {
  int i = 0;
  int runs = 0;
  do {
    runs += 1;
    i += 1;
    if (i == 4) {
      continue;
    }
  } while (i < 4);
  return runs;
}

static int nested(void) {
  int oi = 0;
  int runs = 0;
  do {
    oi += 1;
    int ii = 0;
    do {
      runs += 1;
      ii += 1;
      if (ii == 3) {
        continue;
      }
    } while (ii < 3);
  } while (oi < 2);
  return runs;
}

int main(void) {
  assert(run() == 4);
  assert(nested() == 6);
  return 0;
}
