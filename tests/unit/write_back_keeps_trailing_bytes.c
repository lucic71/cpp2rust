#include <assert.h>
#include <stdlib.h>
#include <string.h>

struct hdr {
  int n;
  char name[1];
};

int main(void) {
  struct hdr *h = calloc(1, sizeof(struct hdr) + 8);
  memcpy(h->name, "abcdefg", 8);

  h->n = 5;

  assert(h->n == 5);
  assert(strcmp(h->name, "abcdefg") == 0);

  free(h);
  return 0;
}
