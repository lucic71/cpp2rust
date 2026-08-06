#include <assert.h>

int scan(const char *s, int start_inside) {
  int depth = 0;
  int seen = 0;
  int i = 0;

  if (start_inside)
    goto inside;

  while (s[i]) {
    if (s[i] == '(') {
      i++;
    inside:
      depth++;
      seen++;
      if (depth > 3)
        break;
      continue;
    }
    i++;
  }
  return depth * 10 + seen;
}

int main(void) {
  assert(scan("", 0) == 0);
  assert(scan("(()", 0) == 22);
  assert(scan("ab(cd", 0) == 11);
  assert(scan("", 1) == 11);
  assert(scan("((((((", 0) == 44);
  return 0;
}
