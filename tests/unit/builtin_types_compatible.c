#include <assert.h>

int main(void) {
  assert(__builtin_types_compatible_p(int, int) == 1);
  assert(__builtin_types_compatible_p(int, long) == 0);

  int x = 0;
  assert(__builtin_types_compatible_p(__typeof__(x), int) == 1);
  assert(__builtin_types_compatible_p(__typeof__(x), long) == 0);

  int *p = 0;
  assert(__builtin_types_compatible_p(__typeof__(p), int *) == 1);
  assert(__builtin_types_compatible_p(__typeof__(p), long *) == 0);

  unsigned long ul = 0;
  assert(__builtin_types_compatible_p(__typeof__(ul), unsigned long) == 1);
  assert(__builtin_types_compatible_p(__typeof__(ul), signed long) == 0);

  return 0;
}
