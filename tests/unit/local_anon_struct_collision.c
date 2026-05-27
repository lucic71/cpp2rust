#include <assert.h>

int first(void) {
  struct {
    int x;
    int y;
  } p;
  p.x = 1;
  p.y = 2;
  return p.x + p.y;
}

int second(void) {
  struct {
    long a;
    long b;
  } q;
  q.a = 10;
  q.b = 20;
  return (int)(q.a + q.b);
}

int main(void) {
  assert(first() == 3);
  assert(second() == 30);
  return 0;
}
