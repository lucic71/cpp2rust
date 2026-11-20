// ADDITIONAL_COMPILE_FLAGS: -std=c++23
#include <cassert>

// N4950 [basic.start.static]/2: a variable with static storage duration that
// is not constant-initialized is zero-initialized before any dynamic
// initialization.
namespace p2_static_storage_is_zero_initialized {
struct Pair {
  int first;
  int second;
};

struct Partial {
  int set;
  int unset;
  Partial() : set(4) {}
};

Pair global_pair;
int global_numbers[3];
int *global_pointer;
Partial global_partial;

int read_static_local() {
  static Pair local_pair;
  static Partial local_partial;
  return local_pair.first + local_pair.second + local_partial.unset;
}

void test() {
  assert(global_pair.first == 0 && global_pair.second == 0);
  assert(global_numbers[0] == 0 && global_numbers[2] == 0);
  assert(global_pointer == nullptr);
  assert(global_partial.set == 4 && global_partial.unset == 0);
  assert(read_static_local() == 0);
}
} // namespace p2_static_storage_is_zero_initialized

int main() {
  p2_static_storage_is_zero_initialized::test();
  return 0;
}
