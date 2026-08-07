#include <assert.h>
#include <string.h>

struct Handler {
  int tag;
  void (*fn)(void);
};

int main(void) {
  int data = 42;
  void *p = &data;

  struct Handler a;
  memset(&a, 0, sizeof(a));
  memcpy(&a.fn, &p, sizeof(p));

  struct Handler b;
  memcpy(&b, &a, sizeof(a));

  assert(b.tag == 0);
  assert(b.fn != 0);
  assert(data == 42);
  return 0;
}
