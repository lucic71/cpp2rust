// panic-ub: refcount
// nondet-result: unsafe
#include <stdlib.h>
#include <string.h>

// A callee overwrites the saved pointer with one into its own buffer, frees
// that buffer, and the caller then subtracts against the stale field.
struct Ctx {
  const char *mark;
};

int main(void) {
  static const char text[] = "hello world";
  struct Ctx c;

  c.mark = &text[0];
  char *tmp = malloc(8);
  memcpy(tmp, "abcdefg", 8);
  c.mark = tmp + 2;
  free(tmp);
  return (int)(&text[6] - c.mark);
}
