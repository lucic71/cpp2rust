#include <assert.h>

enum Mode {
  MODE_NONE,
  MODE_ONE,
  MODE_TWO,
};

struct Config {
  int count;
  enum Mode mode;
};

static struct Config config;

int main(void) {
  assert(config.count == 0);
  assert(config.mode == MODE_NONE);
  return 0;
}
