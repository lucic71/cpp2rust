#include <assert.h>

enum color { RED, GREEN, BLUE };

int main() {
  enum color c = BLUE;
  c++;
  assert(c == (enum color)3);
  assert(c != RED);
  return c == (enum color)3 ? 0 : 1;
}
