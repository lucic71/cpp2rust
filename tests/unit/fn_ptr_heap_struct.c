#include <assert.h>
#include <stdlib.h>

static int total = 0;

static void bump(int by) { total += by; }

static void reset(int ignored) {
  (void)ignored;
  total = 0;
}

struct handlers {
  void (*cb)(int);
  int n;
};

int main(void) {
  struct handlers *h = calloc(1, sizeof(*h));
  assert(h);
  assert(h->cb == NULL);

  h->cb = bump;
  h->n = 7;

  assert(h->cb == bump);
  assert(h->cb != reset);

  h->cb(3);
  assert(total == 3);
  h->cb(4);
  assert(total == 7);

  h->cb = reset;
  h->cb(0);
  assert(total == 0);
  assert(h->n == 7);

  h->cb = NULL;
  assert(h->cb == NULL);

  free(h);
  return 0;
}
