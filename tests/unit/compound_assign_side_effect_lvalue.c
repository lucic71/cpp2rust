#include <assert.h>
#include <string.h>

int main(void) {
  unsigned char buf[8];
  memset(buf, 0, sizeof(buf));
  buf[0] = 1;
  buf[1] = 2;

  unsigned char *p = buf;
  *p++ |= 0x80;
  assert(p == buf + 1);
  assert(buf[0] == 0x81);
  assert(buf[1] == 2);

  unsigned char *r = buf;
  *++r |= 0x10;
  assert(r == buf + 1);
  assert(buf[1] == 0x12);

  unsigned int words[4] = {1, 2, 3, 4};
  unsigned int *w = words;
  *w++ += 10u;
  assert(w == words + 1);
  assert(words[0] == 11);
  assert(words[1] == 2);

  unsigned char *ptrs[2];
  ptrs[0] = buf;
  ptrs[1] = buf;
  unsigned char **pp = ptrs;
  *pp++ += 3;
  assert(pp == ptrs + 1);
  assert(ptrs[0] == buf + 3);
  assert(ptrs[1] == buf);

  unsigned char *q = buf;
  int v = (*q++ |= 0x40);
  assert(q == buf + 1);
  assert(v == 0xc1);
  assert(buf[0] == 0xc1);

  return 0;
}
