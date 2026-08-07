#include <assert.h>

int main(void) {
  char buf[2];
  char *p = buf, *q;
  q = p += 1;
  assert(q == buf + 1);

  char src[] = {'a', 'b'};
  char dst[] = {'x', 'y'};
  char *s = src, *d = dst;
  char last = *d++ = *s++;
  assert(last == 'a');
  assert(d == dst + 1 && s == src + 1);
  assert(dst[0] == 'a' && dst[1] == 'y');

  char out;
  switch (out = 'x') {
  case 'x':
    assert(1);
    break;
  default:
    assert(0);
    break;
  }
  assert(out == 'x');
  return 0;
}
