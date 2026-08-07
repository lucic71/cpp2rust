#include <assert.h>
#include <string.h>

static char buf[32];
static int n;

static void emit(char ch) { buf[n++] = ch; }

static void step(int c, int *last) {
  switch (c) {
  case ')':
    if (*last == 0)
      goto ESCAPE;
    goto COPY;
  case '(':
    *last = '(';
    /* fall through */
  case '.':
  COPY:
    emit(c);
    *last = c;
    break;
  case '^':
    if (*last == '(')
      goto COPY;
    /* fall through */
  default:
    if (c == 'x' || c == 'y') {
    ESCAPE:
      emit('\\');
    }
    emit(c);
    *last = 0xff;
    break;
  }
}

static const char *convert(const char *s) {
  int last = 0;
  n = 0;
  while (*s)
    step(*s++, &last);
  buf[n] = 0;
  return buf;
}

int main(void) {
  assert(strcmp(convert(")a"), "\\)a") == 0);
  assert(strcmp(convert("(.x"), "(.\\x") == 0);
  assert(strcmp(convert("(^"), "(^") == 0);
  assert(strcmp(convert("a^"), "a^") == 0);
  assert(strcmp(convert("()"), "()") == 0);
  assert(strcmp(convert("^x"), "^\\x") == 0);
  assert(strcmp(convert(")("), "\\)(") == 0);
  return 0;
}
