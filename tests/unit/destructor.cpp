#include <cassert>

int global = 0;

struct S {
  ~S() {
    global++;
  }
};

int main() {
  { S s{}; }
  assert(global == 1);

  { S s{}; }
  assert(global == 2);

  return 0;
}
