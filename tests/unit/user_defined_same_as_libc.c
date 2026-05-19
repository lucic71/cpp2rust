#include <assert.h>
#include <stddef.h>

char *strchr(const char *s, int c) { return NULL; }

int main() {
  const char *p = strchr("hello", 'l');
  assert(p == NULL);
  return 0;
}
