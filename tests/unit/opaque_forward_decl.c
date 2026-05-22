#include <stddef.h>

struct opaque;

struct container {
  struct opaque *p;
  int x;
};

int main(void) {
  struct container c = {NULL, 42};
  (void)c.p;
  return c.x - 42;
}
