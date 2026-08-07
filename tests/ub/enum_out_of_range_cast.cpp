#include <assert.h>

enum Color { RED, GREEN, BLUE };

int main() {
  int n = 3;
  Color c = (Color)n;
  assert(c == (Color)3);
  assert(c != BLUE);
  return c == (Color)3 ? 0 : 1;
}
