#include <assert.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>

struct shape_a {
  uint16_t code;
  char pad[14];
};

struct shape_b {
  uint16_t code;
  uint16_t lo;
  uint32_t hi;
  char fill[8];
};

struct shape_c {
  uint16_t code;
  unsigned f1 : 1;
  unsigned f2 : 3;
  unsigned f3 : 12;
};

struct Container {
  union {
    struct shape_a a;
    struct shape_b b;
    struct shape_c c;
    char raw[256];
  } view;
};

int main(void) {
  struct Container c;

  memset(&c, 0, sizeof(c));
  assert(c.view.a.code == 0);
  assert(c.view.b.lo == 0);
  assert(c.view.raw[0] == 0);
  assert(c.view.raw[255] == 0);

  unsigned char src[16] = {0};
  src[0] = 2;
  src[2] = 0x50;
  src[3] = 0x00;
  src[4] = 0x7F;
  src[5] = 0x00;
  src[6] = 0x00;
  src[7] = 0x01;
  size_t len = 16;
  assert(len <= sizeof(c.view.raw));
  memcpy(&c.view.raw, src, len);

  assert(c.view.b.code == 2);
  assert(((unsigned char *)&c.view.b.lo)[0] == 0x50);

  memset(&c, 0, sizeof(c));
  assert(c.view.b.code == 0);

  assert(sizeof(struct shape_c) == 4);
  assert(c.view.c.f1 == 0 && c.view.c.f2 == 0 && c.view.c.f3 == 0);

  c.view.c.code = 2;
  c.view.c.f1 = 1;
  c.view.c.f2 = 5;
  c.view.c.f3 = 0xABC;
  assert(((unsigned char *)&c.view.raw)[2] == 0xCB);
  assert(((unsigned char *)&c.view.raw)[3] == 0xAB);

  memset(&c.view.raw[2], 0xFF, 2);
  assert(c.view.c.f1 == 1 && c.view.c.f2 == 7 && c.view.c.f3 == 0xFFF);
  assert(c.view.c.code == 2);

  memset(&c, 0, sizeof(c));
  assert(c.view.c.f1 == 0 && c.view.c.f2 == 0 && c.view.c.f3 == 0);
  assert(c.view.c.code == 0);

  return 0;
}
