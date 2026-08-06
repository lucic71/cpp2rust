// no-compile: refcount
#include <assert.h>

typedef void (*destructor_t)(void *);

#define D_SENTILEN_1 ((destructor_t)0)
#define D_SENTILEN_2 ((destructor_t)-1)

static int freed = 0;

static void real_free(void *p) {
  (void)p;
  freed++;
}

static int consume(void *data, destructor_t d) {
  if (d == D_SENTILEN_1)
    return 1;
  if (d == D_SENTILEN_2)
    return 2;
  d(data);
  return 3;
}

int main(void) {
  int x = 7;
  assert(consume(&x, D_SENTILEN_1) == 1);
  assert(consume(&x, D_SENTILEN_2) == 2);
  assert(consume(&x, real_free) == 3);
  assert(freed == 1);
  return 0;
}
