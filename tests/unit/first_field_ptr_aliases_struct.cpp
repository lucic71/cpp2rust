#include <assert.h>
#include <map>
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

struct tagged {
  char errbuf[32];
  int code;
  std::map<int, int> lookup;
};

int main(void) {
  struct holder *h = (struct holder *)malloc(sizeof(struct holder));
  h->xfer = (struct transfer *)malloc(sizeof(struct transfer));
  h->xfer->code = 7;
  h->err = h->xfer->errbuf;

  memcpy(h->err, "boom", 5);

  assert(strcmp(h->xfer->errbuf, "boom") == 0);
  assert(h->xfer->code == 7);

  free(h->xfer);
  free(h);

  tagged t;
  t.code = 9;
  t.lookup[1] = 100;
  char *err = t.errbuf;

  memcpy(err, "bang", 5);

  assert(strcmp(t.errbuf, "bang") == 0);
  assert(t.code == 9);
  assert(t.lookup[1] == 100);

  return 0;
}
