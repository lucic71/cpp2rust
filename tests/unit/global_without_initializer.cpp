#include <assert.h>
#include <stdio.h>

struct S {
  int a;
};

S *s;
FILE *file;
size_t size;

int main() {
  assert(s == nullptr);
  assert(file == nullptr);
  assert(size == 0);
  return 0;
};
