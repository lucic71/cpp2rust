#include <assert.h>
#include <stdint.h>
#include <string.h>

int main(void) {
  uint32_t words[4] = {0};
  unsigned char *bytes = (unsigned char *)words;

  for (int i = 0; i < 6; i++) {
    bytes[6 + i] = (unsigned char)(i + 1);
  }
  bytes[12] = 0xAA;

  memmove(bytes + 7, bytes + 6, 6);

  assert(bytes[6] == 1);
  assert(bytes[7] == 1);
  assert(bytes[8] == 2);
  assert(bytes[9] == 3);
  assert(bytes[10] == 4);
  assert(bytes[11] == 5);
  assert(bytes[12] == 6);
  assert(bytes[13] == 0);

  unsigned char src[7] = {9, 8, 7, 6, 5, 4, 0x5A};
  memcpy(bytes + 2, src, 7);

  assert(bytes[1] == 0);
  assert(bytes[2] == 9);
  assert(bytes[7] == 4);
  assert(bytes[8] == 0x5A);
  assert(bytes[9] == 3);

  return 0;
}
