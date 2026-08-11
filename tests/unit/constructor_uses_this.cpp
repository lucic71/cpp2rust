#include <cassert>
#include <vector>

static void append(std::vector<int> *out, int v) { out->push_back(v); }

struct Setup {
  int size;
  std::vector<int> values;

  Setup() : size(0) {
    init();
    append(&values, 7);
  }

  void init() { size = 3; }
};

int main() {
  Setup s;
  assert(s.size == 3);
  assert(s.values.size() == 1);
  assert(s.values[0] == 7);
  return 0;
}
