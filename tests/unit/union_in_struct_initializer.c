#include <assert.h>

struct Item {
  int kind;
  int (*handler)(int);
  union {
    struct Item *next;
    long tag;
  } u;
};

static int double_it(int x) { return x * 2; }
static int negate(int x) { return -x; }

static struct Item items[] = {
    {1, double_it, {0}},
    {2, negate, {0}},
};

int main(void) {
  assert(items[0].handler(21) == 42);
  assert(items[1].handler(21) == -21);
  assert(items[0].u.next == 0);
  items[0].u.tag = 7;
  assert(items[0].u.tag == 7);
  return 0;
}
