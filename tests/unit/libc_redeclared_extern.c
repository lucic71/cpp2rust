// XFAIL: refcount
#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

extern int isatty(int);
extern size_t strlen(const char *);
extern int fileno(FILE *);
extern FILE *popen(const char *, const char *);
extern int pclose(FILE *);

int fileno(FILE *stream) {
  (void)stream;
  return 42;
}

struct sink {
  FILE *in;
  int (*closer)(FILE *);
};

int main(void) {
  assert(fileno(stdout) == 42);

  const char *s = "hello";

  assert(strlen(s) == 5);
  assert(strlen("") == 0);

  int tty = isatty(1);
  assert(tty == 0);

  struct sink k;
  k.in = popen("exit 7", "r");
  assert(k.in != 0);
  k.closer = pclose;
  assert(k.closer(k.in) == 7 * 256);

  k.in = fopen("/dev/null", "r");
  assert(k.in != 0);
  k.closer = fclose;
  assert(k.closer(k.in) == 0);

  return 0;
}
