// no-compile: refcount
#include <assert.h>
#include <string.h>

int main() {
  const char *s = "hello world";
  char *r = strchr(s, 'w');
  assert(r != NULL);
  assert(*r == 'w');
  assert(strchr(s, 'z') == NULL);
  return 0;
}
