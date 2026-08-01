#include <assert.h>
#include <stdlib.h>

struct pair {
  int a;
  int b;
};

static int bump(struct pair *s) {
  s->b += 10;
  return s->b;
}

int main(void) {
  struct pair *s = calloc(1, sizeof(struct pair));
  s->b = 1;

  s->a = bump(s);

  assert(s->a == 11);
  assert(s->b == 11);

  free(s);
  return 0;
}
