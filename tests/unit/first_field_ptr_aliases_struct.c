#include <assert.h>
#include <stdlib.h>
#include <string.h>

struct transfer {
  char errbuf[32];
  int code;
};

struct holder {
  struct transfer *xfer;
  char *err;
};

int main(void) {
  struct holder *h = malloc(sizeof(struct holder));
  h->xfer = malloc(sizeof(struct transfer));
  h->xfer->code = 7;
  h->err = h->xfer->errbuf;

  memcpy(h->err, "boom", 5);

  assert(strcmp(h->xfer->errbuf, "boom") == 0);
  assert(h->xfer->code == 7);

  free(h->xfer);
  free(h);
  return 0;
}
