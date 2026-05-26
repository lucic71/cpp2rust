#include <assert.h>

static int same_name_different_type = 1;
static int same_name_same_type = 5;

int a_foo() { return same_name_different_type; }
int a_bar() { return same_name_same_type; }

float b_foo();
int b_bar();

int main(void) {
  assert(a_foo() == 1);
  assert(b_foo() == 1.0f);
  assert(a_bar() == 5);
  assert(b_bar() == 6);
  return 0;
}
