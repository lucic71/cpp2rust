// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// Tests reset() on unique_ptr fields reached through a struct pointer.
#include <cassert>
#include <memory>

struct Wrapper {
  std::unique_ptr<int> single;
  std::unique_ptr<unsigned char[]> array;
};

void setup(Wrapper *w, int value) {
  w->single.reset(new int(value));
  w->array.reset(new unsigned char[value]);
}

void clear(Wrapper *w) {
  w->single.reset(nullptr);
  w->array.reset(nullptr);
}

int main() {
  Wrapper w;
  setup(&w, 3);
  w.array[0] = (unsigned char)*w.single;
  assert(w.array[0] == 3);
  clear(&w);
  assert(w.single.get() == nullptr);
  return 0;
}
