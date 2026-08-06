#include <assert.h>

static int lookup(char c, int fallback) {
  static const struct Choice {
    char key;
    int op;
  } aChoice[] = {
      {'a', 11},
      {'b', 22},
  };
  int i;
  int r = fallback;

  for (i = 0; i < 2; i++) {
    if (c == aChoice[i].key) {
      r = aChoice[i].op;
      goto done;
    }
  }
done:
  return r;
}

static int classify(int mode, int v) {
  static const struct Weight {
    int lo;
    int hi;
  } aWeight[] = {
      {1, 2},
      {3, 4},
  };
  int r = 0;

  if (v > 0) {
  positive:
    r = aWeight[0].lo + v;
    if (mode == 1)
      goto negative;
  } else {
    if (mode == 2)
      goto positive;
  negative:
    r = aWeight[1].hi - v;
  }
  return r;
}

int main(void) {
  assert(lookup('a', -1) == 11);
  assert(lookup('b', -1) == 22);
  assert(lookup('z', -1) == -1);

  assert(classify(0, 5) == 6);
  assert(classify(1, 5) == -1);
  assert(classify(0, -3) == 7);
  assert(classify(2, -3) == -2);
  return 0;
}
