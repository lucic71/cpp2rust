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

int main(void) {
  const char *high = "\x81\xff\xc4";

  char buf[32];
  assert(snprintf(buf, sizeof(buf), "[%s]%c", high, 0xe4) == 6);
  assert(memcmp(buf, "[\x81\xff\xc4]\xe4", 6) == 0);
  assert(buf[6] == 0);

  const char *path = "cpp2rust_high_bytes.tmp";
  FILE *fp = fopen(path, "wb");
  assert(fp != NULL);
  assert(emit(fp, "%s%c\n", high, 0x80) == 5);
  assert(fclose(fp) == 0);

  fp = fopen(path, "rb");
  assert(fp != NULL);
  char rd[16] = {0};
  assert(fread(rd, 1, sizeof(rd), fp) == 5);
  assert(memcmp(rd, "\x81\xff\xc4\x80\n", 5) == 0);
  assert(fclose(fp) == 0);
  assert(unlink(path) == 0);

  return 0;
}
