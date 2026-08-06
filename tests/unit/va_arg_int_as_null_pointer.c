#include <assert.h>
#include <stdarg.h>

struct handle {
  int value;
};

static int configure(struct handle *h, int op, ...) {
  va_list ap;
  int rc = 0;
  va_start(ap, op);
  int onoff = va_arg(ap, int);
  int *pOut = va_arg(ap, int *);
  h->value = onoff;
  if (pOut) {
    *pOut = onoff;
    rc = 1;
  }
  va_end(ap);
  return rc;
}

int main(void) {
  struct handle h = {0};
  assert(configure(&h, 7, 1, 0) == 0);
  assert(h.value == 1);

  int out = -1;
  assert(configure(&h, 7, 5, &out) == 1);
  assert(out == 5);
  assert(h.value == 5);
  return 0;
}
