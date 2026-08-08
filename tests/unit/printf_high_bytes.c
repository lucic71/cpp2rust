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

  assert(snprintf(buf, sizeof(buf), "%.*s", 3, high) == 3);
  assert(memcmp(buf, "\x81\xff\xc4", 3) == 0);
  assert(buf[3] == 0);

  assert(snprintf(buf, sizeof(buf), "[%.*s]", 2, high) == 4);
  assert(memcmp(buf, "[\x81\xff]", 4) == 0);

  assert(snprintf(buf, sizeof(buf), "%.2s", high) == 2);
  assert(memcmp(buf, "\x81\xff", 2) == 0);

  assert(snprintf(buf, sizeof(buf), "%.16s", high) == 3);
  assert(memcmp(buf, "\x81\xff\xc4", 3) == 0);

  const char unterminated[3] = {'\x81', '\xff', '\xc4'};
  assert(snprintf(buf, sizeof(buf), "%.*s", 3, unterminated) == 3);
  assert(memcmp(buf, "\x81\xff\xc4", 3) == 0);

  const char *path = "cpp2rust_high_bytes.tmp";
  FILE *fp = fopen(path, "wb");
  assert(fp != NULL);
  assert(emit(fp, "%s%c%.*s\n", high, 0x80, 2, high) == 7);
  assert(fclose(fp) == 0);

  fp = fopen(path, "rb");
  assert(fp != NULL);
  char rd[16] = {0};
  assert(fread(rd, 1, sizeof(rd), fp) == 7);
  assert(memcmp(rd, "\x81\xff\xc4\x80\x81\xff\n", 7) == 0);
  assert(fclose(fp) == 0);
  assert(unlink(path) == 0);

  return 0;
}
