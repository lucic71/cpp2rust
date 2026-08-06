#include <assert.h>

struct S {
  int x;
  int *p;
  struct S *self;
};

int main(void) {
  struct S s;
  s.x = 1;
  s.p = &s.x;
  *s.p = 5;
  assert(s.x == 5);

  *s.p = s.x + 1;
  assert(s.x == 6);

  s.self = &s;
  s.self->x = 7;
  assert(s.x == 7);
  return 0;
}
