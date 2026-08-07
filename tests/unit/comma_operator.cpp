#include <cassert>

int main() {
  int x = 1;
  int y = (x = 2, x + 1);
  assert(x == 2);
  assert(y == 3);

  int z = (1, 2, 3);
  assert(z == 3);

  int counter = 0;
  int w = (counter++, counter++, counter);
  assert(counter == 2);
  assert(w == 2);

  int a = 0, b = 0;
  if ((a = 1, b = 2, a + b > 0)) {
    assert(a == 1);
    assert(b == 2);
  }

  unsigned short hf = 0;
  double d = 0.5;
  if (hf = (unsigned short)(d * 4), d == (double)hf / 4) {
    assert(hf == 2);
  } else {
    assert(0);
  }

  int n = 0;
  while (n = n + 1, n < 3) {
  }
  assert(n == 3);

  return 0;
}
