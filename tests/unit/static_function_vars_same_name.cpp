#include <assert.h>

int a() {
  static int i = 1;
  return i;
}

int b() {
  static int i = 2;
  return i;
}

int main() {
  assert(a() == 1);
  assert(b() == 2);
  return 0;
}
