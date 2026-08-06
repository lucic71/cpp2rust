#include <assert.h>

static int pick(const char *, const char *, int);
static int total(int *, int *);

static int pick(const char *a, const char *b, int n) {
  return (a == b ? 10 : 20) + n + (a[0] - 'a');
}

static int total(int *x, int *y) {
  *x += 1;
  return *x + *y;
}

int main(void) {
  const char *s = "abc";
  const char *t = "bcd";
  int n = 5;
  int v = 4;

  assert(pick(s, s, n) == 15);
  assert(pick(s, t, n) == 25);
  assert(total(&v, &v) == 10);
  assert(v == 5);
  return 0;
}
