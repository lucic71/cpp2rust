#include <assert.h>
#include <stdlib.h>

int main(void) {
  char *end;
  const char *s;

  s = "42";
  assert(strtoll(s, &end, 10) == 42 && end - s == 2);
  s = "  -17abc";
  assert(strtoll(s, &end, 10) == -17 && end - s == 5);
  s = "0xff";
  assert(strtoll(s, &end, 16) == 255 && end - s == 4);
  assert(strtoll(s, &end, 0) == 255 && end - s == 4);
  s = "0755";
  assert(strtoll(s, &end, 0) == 493 && end - s == 4);
  s = "0x";
  assert(strtoll(s, &end, 16) == 0 && end - s == 1);
  s = "9223372036854775808";
  assert(strtoll(s, &end, 10) == 9223372036854775807LL && end - s == 19);
  s = "-9223372036854775809";
  assert(strtoll(s, &end, 10) == (-9223372036854775807LL - 1) && end - s == 20);
  s = "junk";
  assert(strtoll(s, &end, 10) == 0 && end == s);
  s = "z";
  assert(strtoll(s, &end, 36) == 35 && end - s == 1);
  assert(strtoll("55", NULL, 10) == 55);

  s = "3.14";
  assert(strtod(s, &end) == 3.14 && end - s == 4);
  s = "  -2.5e3xyz";
  assert(strtod(s, &end) == -2500.0 && end - s == 8);
  s = "1.e5";
  assert(strtod(s, &end) == 100000.0 && end - s == 4);
  s = ".5";
  assert(strtod(s, &end) == 0.5 && end - s == 2);
  s = "1e";
  assert(strtod(s, &end) == 1.0 && end - s == 1);
  s = "junk";
  assert(strtod(s, &end) == 0.0 && end == s);
  assert(strtod("+0.375e-1", NULL) == 0.0375);
  return 0;
}
