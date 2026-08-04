#include <assert.h>
#include <stdlib.h>

struct inner {
  int x;
  int y;
};

struct outer {
  struct inner in;
  int total;
};

static int read_total(struct outer *o) { return o->total; }

int main(void) {
  struct outer o = {{1, 2}, 10};
  struct outer *p = &o;
  struct outer *q = &o;

  p->total = q->in.x + q->in.y;
  assert(o.total == 3);

  struct inner *ip = &p->in;
  ip->x = p->total + 1;
  assert(o.in.x == 4);

  p->total += q->in.x;
  assert(o.total == 7);

  p->in.y = read_total(q);
  assert(o.in.y == 7);

  struct outer *h = malloc(sizeof(struct outer));
  struct outer *ha = h;
  h->total = 5;
  h->in.x = 1;
  ha->total = h->total + ha->in.x;
  assert(h->total == 6);
  free(h);

  return 0;
}
