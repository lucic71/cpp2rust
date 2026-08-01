#include <assert.h>
#include <stdlib.h>

struct big {
  long a;
  long b;
};

struct outer {
  long pad;
  struct big big;
};

struct holder {
  struct big *p;
};

int main(void) {
  struct outer *o = malloc(sizeof(struct outer));
  o->pad = 1;
  o->big.a = 2;
  o->big.b = 3;

  struct holder *h = malloc(sizeof(struct holder));
  h->p = &o->big;

  assert(h->p->a == 2);
  h->p->b = 9;
  assert(o->big.b == 9);
  assert(o->pad == 1);

  free(h);
  free(o);
  return 0;
}
