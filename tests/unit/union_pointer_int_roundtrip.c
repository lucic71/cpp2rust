#include <assert.h>
#include <stdint.h>

#define INT_TO_PTR(X) ((void *)(intptr_t)(X))
#define PTR_TO_INT(P) ((int)(intptr_t)(P))

struct cb {
  void *ctx;
};

int main(void) {
  int arr[4] = {10, 20, 30, 40};

  union {
    int *p;
    uintptr_t bits;
  } u;

  u.p = &arr[1];
  u.bits += 2 * sizeof(int);
  int *q = u.p;

  assert(*q == 40);
  assert(q == &arr[3]);

  u.bits -= 3 * sizeof(int);
  assert(u.p == &arr[0]);
  assert(*u.p == 10);

  u.p = arr + 4;
  assert(u.p == arr + 4);

  struct cb c = {INT_TO_PTR(99)};
  assert(PTR_TO_INT(c.ctx) == 99);

  void *m = INT_TO_PTR(-1);
  assert(PTR_TO_INT(m) == -1);
  assert(m != 0);

  c.ctx = INT_TO_PTR(0);
  assert(c.ctx == 0);

  return 0;
}
