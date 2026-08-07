// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cassert>
#include <cstdio>
#include <string>

std::string fn(std::string v) { return v + " str"; }

const std::string &fn2(const std::string &v) { return v; }

int main() {
  fprintf(stdout, "%s\n", "fprintf stdout");
  fprintf(stdout, "%d %u %ld\n", 1, 2U, 3L);
  fprintf(stdout, "hello world");
  FILE *in = stdin;
  assert(in != NULL);
  printf("%s\n", "printf");
  printf("hello world");
  std::string s = "a string";
  printf("%s\n", s.data());
  printf("%s\n", fn("foo").c_str());
  printf("%s\n", fn2(s).c_str());

  int n = printf("%s", "1234");
  assert(n == 4);
  printf("\n");

  int total = 0;
  total += printf("%d", 42);
  total += printf("%c", 'x');
  printf("\n");
  assert(total == 3);

  while (n++ < 6) {
    printf(" ");
  }
  assert(n == 7);
  printf("\n");
  return 0;
}
