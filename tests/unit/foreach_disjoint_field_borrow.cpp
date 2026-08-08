// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// Test that iterating over one struct field while mutating another
// does not cause a refcell double borrow error.
#include <assert.h>
#include <vector>

struct S {
  std::vector<std::vector<int>> parts;
  int a;
};

int main() {
  S s;
  s.a = 0;
  s.parts.resize(3);
  s.parts[2].resize(2);

  int points = 0;
  S *p = &s;
  for (const auto &part : p->parts) {
    points += part.size();
    s.a++;
  }
  assert(s.a == 3);
  assert(points == 2);

  return 0;
}
