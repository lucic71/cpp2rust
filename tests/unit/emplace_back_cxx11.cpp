// ADDITIONAL_COMPILE_FLAGS: -std=c++11
#include <cassert>
#include <deque>
#include <vector>

struct Pair {
  int first;
  int second;
  Pair(int a, int b) : first(a), second(b * 2) {}
};

int main() {
  std::vector<Pair> pairs;
  pairs.emplace_back(1, 2);
  assert(pairs.size() == 1);
  assert(pairs[0].first == 1 && pairs[0].second == 4);

  std::vector<std::vector<int>> nested;
  nested.emplace_back(3, 5);
  assert(nested.size() == 1);
  assert(nested[0].size() == 3 && nested[0][2] == 5);

  std::deque<Pair> queue;
  queue.emplace_back(6, 7);
  assert(queue.front().first == 6 && queue.front().second == 14);

  return 0;
}
