#include <assert.h>
#include <stdint.h>
#include <string.h>

struct holder {
  uint8_t mask[4];
  uint32_t after;
};

static void encode(struct holder *h, uint8_t *out) {
  *(uint8_t *)&h->mask = 7;
  memcpy(out, &h->mask, sizeof(h->mask));
}

int main(void) {
  struct holder h = {{1, 2, 3, 4}, 0x55667788};
  uint8_t out[4];

  encode(&h, out);

  assert(out[0] == 7);
  assert(out[3] == 4);
  assert(h.after == 0x55667788);
  return 0;
}
