#include <assert.h>
#include <stdlib.h>

struct region {
  void *start;
  void *mid;
  void *end;
};

static int in_low_half(struct region *r, void *p) {
  return p >= r->start && p < r->mid;
}

int main(void) {
  char *buf = malloc(64);
  struct region r;
  r.start = buf;
  r.mid = buf + 32;
  r.end = buf + 64;

  assert(r.start < r.mid);
  assert(r.mid < r.end);

  assert(in_low_half(&r, buf + 10));
  assert(!in_low_half(&r, buf + 40));
  assert(in_low_half(&r, buf));
  assert(!in_low_half(&r, buf + 32));

  char *other = malloc(64);
  void *op = other;
  assert(!(op >= r.start && op < r.end));

  free(other);
  free(buf);
  return 0;
}
