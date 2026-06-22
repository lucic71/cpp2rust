#include <assert.h>

struct S {
  int head;
  int tail[3];
  char buf[4];
};

struct S s = {5};

int main() {
  assert(s.head == 5);
  for (int i = 0; i < 3; i++) {
    assert(s.tail[i] == 0);
  }
  for (int i = 0; i < 4; i++) {
    assert(s.buf[i] == 0);
  }
  return 0;
}
