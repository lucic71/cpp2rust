#include <assert.h>
#include <stdlib.h>
#include <string.h>

struct payload {
  int value;
};

struct holder {
  struct payload *first;
  struct payload *second;
  int count;
};

int main(void) {
  struct holder *h = malloc(sizeof(struct holder));
  assert(h != NULL);
  h->first = malloc(sizeof(struct payload));
  h->second = malloc(sizeof(struct payload));
  assert(h->first != NULL);
  assert(h->second != NULL);
  h->first->value = 11;
  h->second->value = 22;
  h->count = 2;

  free(h->first);
  h->count = 1;
  assert(h->count == 1);
  assert(h->second->value == 22);

  h->first = NULL;
  assert(h->first == NULL);
  assert(h->count == 1);

  free(h->second);
  h->second = NULL;
  h->count = 0;
  assert(h->count == 0);
  assert(h->first == NULL);

  free(h);
  return 0;
}
