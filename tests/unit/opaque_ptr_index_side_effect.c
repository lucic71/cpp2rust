#include <assert.h>
#include <stdlib.h>

struct bin {
  int idx;
  char buf[8];
};

static void store(void *p, char c) {
  struct bin *b = (struct bin *)p;
  b->buf[b->idx++] = c;
}

int main(void) {
  struct bin *b = calloc(1, sizeof(struct bin));
  store(b, 'a');
  store(b, 'b');

  assert(b->idx == 2);
  assert(b->buf[0] == 'a');
  assert(b->buf[1] == 'b');

  free(b);
  return 0;
}
