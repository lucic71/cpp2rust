#include <assert.h>
#include <stdlib.h>

struct inner {
  long a;
  long b;
};

struct outer {
  struct inner in;
  long tag;
};

struct holder {
  long *words;
  struct inner *field;
};

int main(void) {
  struct outer *o = malloc(sizeof(struct outer));
  o->tag = 7;

  struct holder *h = malloc(sizeof(struct holder));
  h->words = malloc(2 * sizeof(long));
  h->field = &o->in;

  h->words[0] = 11;
  h->words[1] = 22;
  h->field->a = 33;

  assert(h->words[0] == 11);
  assert(h->words[1] == 22);
  assert(h->field->a == 33);
  assert(o->tag == 7);

  free(h->words);
  free(h);
  free(o);
  return 0;
}
