#include <assert.h>
#include <stdio.h>
#include <string.h>

int main() {
  char buf[32];
  assert(snprintf(buf, sizeof(buf), "x=%d y=%u", -3, 7u) == 8);
  assert(strcmp(buf, "x=-3 y=7") == 0);
  assert(snprintf(buf, 4, "%s", "hello") == 5);
  assert(strcmp(buf, "hel") == 0);
  assert(snprintf(buf, sizeof(buf), "%05d|%02x", 42, 255) == 8);
  assert(strcmp(buf, "00042|ff") == 0);
  assert(snprintf(buf, sizeof(buf), "%c%%", 65) == 2);
  assert(strcmp(buf, "A%") == 0);
  assert(snprintf(buf, sizeof(buf), "%ld %lu %zu", -1L, 1UL, (size_t)9) == 6);
  assert(strcmp(buf, "-1 1 9") == 0);
  buf[0] = 'Z';
  assert(snprintf(buf, 0, "%d", 123) == 3);
  assert(buf[0] == 'Z');
  return 0;
}
