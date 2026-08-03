#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

struct pair {
  FILE *a;
  FILE *b;
};

int main(void) {
  struct pair *p = calloc(1, sizeof(struct pair));
  p->a = fopen("/dev/null", "w");
  p->b = fopen("/dev/null", "w");
  assert(p->a != p->b);

  fclose(p->a);
  fclose(p->b);

  for (int i = 0; i < 64; i++) {
    char *q = malloc(16);
    q[0] = (char)i;
    free(q);
  }

  assert(p->a != p->b);

  free(p);
  return 0;
}
