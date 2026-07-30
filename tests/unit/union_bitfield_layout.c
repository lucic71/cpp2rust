// XFAIL: unsafe
#include <assert.h>
#include <string.h>

struct packed_flags {
  unsigned int a : 1;
  unsigned int b : 3;
  unsigned int wide : 20;
  int sgn : 4;
  unsigned int tail;
};

union view {
  struct packed_flags f;
  unsigned char raw[8];
};

int main(void) {
  union view v;
  memset(&v, 0, sizeof(v));

  v.f.a = 1;
  v.f.b = 5;
  v.f.wide = 0xABCDE;
  v.f.sgn = -3;
  v.f.tail = 0x11223344;

  assert(v.raw[0] == 0xEB);
  assert(v.raw[1] == 0xCD);
  assert(v.raw[2] == 0xAB);
  assert(v.raw[3] == 0x0D);
  assert(v.raw[4] == 0x44);
  assert(v.raw[5] == 0x33);
  assert(v.raw[6] == 0x22);
  assert(v.raw[7] == 0x11);

  v.f.b = 2;
  assert(v.raw[0] == 0xE5);
  assert(v.f.a == 1);
  assert(v.f.wide == 0xABCDE);
  assert(v.f.sgn == -3);
  assert(v.f.tail == 0x11223344);

  memset(&v, 0, sizeof(v));
  v.raw[0] = 0x3C;
  v.raw[1] = 0x12;
  v.raw[2] = 0x00;
  v.raw[3] = 0x0F;

  assert(v.f.a == 0);
  assert(v.f.b == 6);
  assert(v.f.wide == 0x00123);
  assert(v.f.sgn == -1);
  assert(v.f.tail == 0);

  return 0;
}
