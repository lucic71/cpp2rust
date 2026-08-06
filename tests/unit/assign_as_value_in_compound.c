#include <assert.h>

struct item {
  unsigned char flags;
};

static unsigned char merge(struct item *a, int n) {
  unsigned char all = 0;
  int i;
  for (i = n - 1; i > 0; i--) {
    all |= a[i].flags = a[i - 1].flags;
  }
  return all;
}

int main(void) {
  struct item a[3];
  a[0].flags = 1;
  a[1].flags = 2;
  a[2].flags = 4;

  assert(merge(a, 3) == 3);
  assert(a[0].flags == 1);
  assert(a[1].flags == 1);
  assert(a[2].flags == 2);

  int x = 0;
  int y = 5;
  int z = 0;
  z += x = y;
  assert(z == 5);
  assert(x == 5);

  unsigned char c = 1;
  int v = (c <<= 3);
  assert(v == 8);
  assert(c == 8);

  int steps = 0;
  c = 1;
  do {
    steps++;
  } while (((c <<= 1) & 0x40) != 0x40);
  assert(steps == 6);
  assert(c == 0x40);

  return 0;
}
