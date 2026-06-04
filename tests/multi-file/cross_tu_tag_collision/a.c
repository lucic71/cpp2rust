#include <assert.h>

struct widget {
  int id;
};

int b_value(void);

int a_value(void) {
  struct widget w;
  w.id = 11;
  return w.id;
}

int main(void) {
  assert(a_value() == 11);
  assert(b_value() == 2);
  return 0;
}
