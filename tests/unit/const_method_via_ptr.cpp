#include <cassert>
#include <deque>
#include <memory>
#include <vector>

struct Item {
  std::unique_ptr<int> value;
};

struct Wrapper {
  std::vector<Item> items;
  std::deque<Item> queue;
};

int count(Wrapper *w, std::deque<Item> *q) {
  return (int)w->items.size() + (w->queue.empty() ? 1 : 0) + (q->empty() ? 2 : 0);
}

int main() {
  Wrapper w;
  assert(count(&w, &w.queue) == 3);
  return 0;
}
