#include <assert.h>
#include <string.h>

typedef struct {
  int tag;
  union {
    const char *p;
    int n;
    char c[4];
  } u;
} item;

static const item items[] = {
    {0, {.p = "xy"}},
    {1, {.n = 42}},
    {2, {.c = "ab"}},
};

int main(void) {
  const item *it = &items[0];
  assert(strcmp(it->u.p, "xy") == 0);
  assert(items[1].u.n == 42);
  assert(items[2].u.c[1] == 'b');
  return 0;
}
