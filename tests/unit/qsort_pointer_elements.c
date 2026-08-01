#include <assert.h>
#include <stdlib.h>
#include <string.h>

static int cmp(const void *a, const void *b) {
  return strcmp(*(const char *const *)a, *(const char *const *)b);
}

int main(void) {
  const char *items[4] = {"pear", "apple", "fig", "date"};

  qsort((void *)items, 4, sizeof(*items), cmp);

  assert(strcmp(items[0], "apple") == 0);
  assert(strcmp(items[1], "date") == 0);
  assert(strcmp(items[2], "fig") == 0);
  assert(strcmp(items[3], "pear") == 0);
  return 0;
}
