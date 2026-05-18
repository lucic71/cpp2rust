#include <assert.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>

struct Entry {
  uint8_t bits;
  uint16_t value;
};

int main(void) {
  struct Entry table[8] = {
      {1, 0x1111}, {2, 0x2222}, {3, 0x3333}, {4, 0x4444},
      {0, 0},      {0, 0},      {0, 0},      {0, 0},
  };
  size_t table_size = 4;

  memcpy(&table[table_size], &table[0], table_size * sizeof(table[0]));

  assert(table[4].bits == 1 && table[4].value == 0x1111);
  assert(table[5].bits == 2 && table[5].value == 0x2222);
  assert(table[6].bits == 3 && table[6].value == 0x3333);
  assert(table[7].bits == 4 && table[7].value == 0x4444);

  return 0;
}
