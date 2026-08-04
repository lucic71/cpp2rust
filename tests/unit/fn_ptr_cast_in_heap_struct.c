#include <assert.h>
#include <stdlib.h>

struct item {
  int value;
};

typedef int (*cb_t)(void *p);

static int read_item(struct item *it) { return it->value + 1; }

struct holder {
  cb_t callback;
};

int main(void) {
  struct holder *h = calloc(1, sizeof(struct holder));
  h->callback = (cb_t)read_item;

  struct item it;
  it.value = 41;
  assert(h->callback(&it) == 42);

  free(h);
  return 0;
}
