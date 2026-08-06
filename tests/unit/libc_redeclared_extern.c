#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

extern int isatty(int);
extern size_t strlen(const char *);
extern int fileno(FILE *);

int fileno(FILE *stream) {
  (void)stream;
  return 42;
}

int main(void) {
  assert(fileno(stdout) == 42);

  const char *s = "hello";

  assert(strlen(s) == 5);
  assert(strlen("") == 0);

  int tty = isatty(1);
  assert(tty == 0);

  return 0;
}
