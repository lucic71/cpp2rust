#include <assert.h>

int acc(int x) {
  static int total = 5;
  static const int limit = 10;
  if (x < 0)
    goto done;
  total += x;
  if (total > limit)
    total = limit;
done:
  return total;
}

int main(void) {
  assert(acc(3) == 8);
  assert(acc(-1) == 8);
  assert(acc(4) == 10);
  return 0;
}
