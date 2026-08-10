#include <cassert>
#include <deque>
#include <memory>

struct Item {
  std::unique_ptr<int> value;
};

struct Holder {
  std::deque<Item> items;
  Item pending;
};

void store(Holder *h) { h->items.emplace_back(std::move(h->pending)); }

int main() {
  Holder h;
  h.pending.value.reset(new int(7));
  store(&h);
  assert(h.pending.value.get() == nullptr);
  assert(*h.items.front().value == 7);
  return 0;
}
