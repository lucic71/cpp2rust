// no-compile: refcount
#define _GNU_SOURCE
#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <sys/types.h>
#include <sys/xattr.h>
#include <unistd.h>

static void test_fsetxattr_bad_fd(void) {
  assert(fsetxattr(-1, "user.test", "v", 1, 0) == -1);
}

static void test_fsetxattr_roundtrip(void) {
  const char *path = "/tmp/cpp2rust_xattr_test.tmp";
  FILE *fp = fopen(path, "wb");
  assert(fp != NULL);
  int fd = fileno(fp);
  assert(fd != -1);
  assert(fsetxattr(fd, "user.test", "value", 5, 0) == 0);
  char buf[16];
  assert(fgetxattr(fd, "user.test", buf, sizeof(buf)) == 5);
  assert(memcmp(buf, "value", 5) == 0);
  assert(fclose(fp) == 0);
  assert(unlink(path) == 0);
}

int main(void) {
  test_fsetxattr_bad_fd();
  test_fsetxattr_roundtrip();
  return 0;
}
