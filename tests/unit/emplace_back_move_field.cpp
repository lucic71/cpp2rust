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

struct Nested {
  std::deque<Item> items;
};

struct Outer {
  Nested nested;
  std::deque<Item> *sink;
  Item pending;
};

void store_through(Outer *o) { o->sink->emplace_back(std::move(o->pending)); }

int main() {
  Holder h;
  h.pending.value.reset(new int(7));
  store(&h);
  assert(h.pending.value.get() == nullptr);
  assert(*h.items.front().value == 7);

  Outer o;
  o.sink = &o.nested.items;
  o.pending.value.reset(new int(9));
  store_through(&o);
  assert(o.pending.value.get() == nullptr);
  assert(*o.nested.items.front().value == 9);

  return 0;
}
