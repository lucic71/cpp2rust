// XFAIL: refcount
#include <assert.h>
#include <stdarg.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

static int emit(FILE *out, const char *fmt, ...) {
  va_list ap;
  va_start(ap, fmt);
  int rc = vfprintf(out, fmt, ap);
  va_end(ap);
  return rc;
}

static int emit_after_skip(FILE *out, const char *fmt, ...) {
  va_list ap;
  va_start(ap, fmt);
  int skipped = va_arg(ap, int);
  int rc = vfprintf(out, fmt, ap);
  va_end(ap);
  return rc + skipped;
}

int main(void) {
  const char *path = "cpp2rust_vfprintf.tmp";
  FILE *fp = fopen(path, "wb");
  assert(fp != NULL);
  assert(emit(fp, "%s=%d\n", "count", 42) == 9);
  assert(emit_after_skip(fp, "%c%d\n", 100, 'x', 7) == 103);
  assert(fclose(fp) == 0);

  fp = fopen(path, "rb");
  assert(fp != NULL);
  char buf[32] = {0};
  assert(fread(buf, 1, 32, fp) == 12);
  assert(memcmp(buf, "count=42\nx7\n", 12) == 0);
  assert(fclose(fp) == 0);
  assert(unlink(path) == 0);
  return 0;
}
