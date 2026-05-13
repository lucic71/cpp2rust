// no-compile: refcount
#include <cassert>
#include <cstring>

int main() {
  const char *s = "hello world";
  const char *r = strchr(s, 'w');
  assert(r != NULL);
  assert(*r == 'w');
  assert(strchr(s, 'z') == NULL);
  return 0;
}
