#include <assert.h>
#include <stdlib.h>

typedef struct base {
  struct base *next;
} base;

typedef struct derived {
  base head;
  size_t value;
} derived;

int main(void) {
  derived *d = malloc(sizeof(*d));
  assert(d != NULL);
  d->head.next = NULL;
  d->value = 7;

  base *b = &d->head;
  derived *back = (derived *)b;
  assert(back == d);
  assert(back->value == 7);

  free(d);
  return 0;
}
