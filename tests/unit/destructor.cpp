#include <cassert>

static int total = 0;

struct Counter {
  int bits = 16;

  int units() const { return bits / 8; }

  ~Counter() { total += units(); }
};

struct Watcher {
  int *target = nullptr;

  ~Watcher() {
    if (target != nullptr) {
      total += *target;
      target = nullptr;
    }
  }
};

struct Owner {
  Watcher watcher;
};

struct Tracker {
  int step = 4;

  ~Tracker();
};

Tracker::~Tracker() { total += step; }

int main() {
  { Counter c; }
  assert(total == 2);

  int value = 40;
  {
    Owner o;
    o.watcher.target = &value;
  }
  assert(total == 42);

  { Tracker t; }
  assert(total == 46);

  return 0;
}
