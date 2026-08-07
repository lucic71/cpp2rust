#include <assert.h>

typedef struct {
  int (*first)(int);
  int (*second)(int);
} ops;

static const ops table;
static const int limits[3];

static int twice(int v) { return v * 2; }

static const ops table = {
    .second = twice,
};

static const int limits[3] = {4, 5, 6};

int main(void) {
  assert(table.first == 0);
  assert(table.second != 0);
  assert(table.second(7) == 14);
  assert(limits[1] == 5);
  return 0;
}
