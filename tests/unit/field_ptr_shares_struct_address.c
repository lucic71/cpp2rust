#include <assert.h>
#include <stdlib.h>

struct conn {
  int first;
  int port;
};

int main(void) {
  struct conn *c = malloc(sizeof(struct conn));
  c->port = 443;

  int *p = &c->first;
  *p = 1;

  assert(c->first == 1);
  assert(c->port == 443);

  free(c);
  return 0;
}
