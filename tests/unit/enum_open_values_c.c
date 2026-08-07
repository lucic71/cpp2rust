#include <assert.h>

enum Flags { FLAG_A = 256, FLAG_B = 512, FLAG_A_ALIAS = 256 };

static enum Flags with_a(enum Flags f) { return (enum Flags)(f | FLAG_A); }

int main(void) {
  enum Flags f = (enum Flags)'x';
  f = with_a(f);
  f |= FLAG_B;
  assert((f & 0xff) == 'x');
  assert((f & ~0xff) == (FLAG_A | FLAG_B));

  enum Flags zero = (enum Flags)0;
  assert(!zero);
  assert(zero != f);

  int as_int = f;
  assert(as_int == (256 | 512 | 'x'));

  f = (enum Flags)(f & ~FLAG_B);
  assert(f == (enum Flags)(256 | 'x'));

  enum Flags seq = FLAG_A;
  seq++;
  assert(seq == (enum Flags)257);
  --seq;
  assert(seq == FLAG_A);

  assert(FLAG_A_ALIAS == FLAG_A);
  enum Flags alias = FLAG_A_ALIAS;
  assert(alias == FLAG_A);
  assert(alias | FLAG_B);

  return 0;
}
