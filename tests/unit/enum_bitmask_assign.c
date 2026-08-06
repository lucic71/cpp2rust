#include <assert.h>

typedef enum { F_NONE = 0, F_A = 1, F_B = 2, F_AB = 3, F_ALL = 7 } Flags;

static Flags add_b(Flags f) {
  f |= F_B;
  return f;
}

int main(void) {
  Flags f = F_A;
  f = add_b(f);
  assert(f == F_AB);

  Flags g = F_NONE;
  g |= F_A;
  assert(g == F_A);

  g |= F_B;
  assert(g == F_AB);

  g &= F_ALL;
  assert(g == F_AB);

  return 0;
}
