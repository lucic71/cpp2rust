#include <assert.h>
#include <stdlib.h>

struct Inner {
  int value;
};

struct Outer {
  struct Inner slots[2];
  struct Inner *cur;
};

static void set_current(struct Outer *p, const int *src) {
  p->cur->value = *src;
}

static void bump_current(struct Outer *p) {
  p->cur->value = p->slots[0].value + 1;
}

int main(void) {
  struct Outer *p = malloc(sizeof(struct Outer));
  int a = 7, b = 8;

  p->slots[0].value = 1;
  p->slots[1].value = 2;

  p->cur = &p->slots[0];
  set_current(p, &a);
  assert(p->slots[0].value == 7);

  p->cur = &p->slots[1];
  set_current(p, &b);
  assert(p->slots[1].value == 8);

  bump_current(p);
  assert(p->slots[1].value == 8);

  free(p);
  return 0;
}
