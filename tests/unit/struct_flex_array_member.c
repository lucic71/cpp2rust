#include <assert.h>
#include <stdlib.h>
#include <string.h>

struct peer {
  int port;
  char hostname[1];
};

static struct peer *peer_create(const char *host) {
  struct peer *p = malloc(sizeof(struct peer) + strlen(host));
  p->port = 443;
  memcpy(p->hostname, host, strlen(host) + 1);
  return p;
}

int main(void) {
  struct peer *p = peer_create("example.com");
  assert(p->port == 443);
  assert(strcmp(p->hostname, "example.com") == 0);
  assert(p->hostname[0] == 'e');
  assert(p->hostname[7] == '.');

  p->hostname[0] = 'E';
  assert(strcmp(p->hostname, "Example.com") == 0);
  assert(p->port == 443);

  char *h = &p->hostname[8];
  assert(strcmp(h, "com") == 0);

  free(p);
  return 0;
}
