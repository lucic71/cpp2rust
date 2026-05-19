// no-compile: refcount
#include <assert.h>
#include <stddef.h>
#include <string.h>
#include <sys/select.h>
#include <sys/time.h>
#include <unistd.h>

static void fd_set_bit(int fd, fd_set *set) {
  unsigned char *p = (unsigned char *)set;
  p[fd / 8] |= (unsigned char)(1 << (fd % 8));
}

static int fd_isset_bit(int fd, const fd_set *set) {
  const unsigned char *p = (const unsigned char *)set;
  return (p[fd / 8] >> (fd % 8)) & 1;
}

static void test_select(void) {
  int fds[2];
  assert(pipe(fds) == 0);
  assert(write(fds[1], "x", 1) == 1);

  fd_set rfds;
  memset(&rfds, 0, sizeof(rfds));
  fd_set_bit(fds[0], &rfds);

  struct timeval tv;
  tv.tv_sec = 0;
  tv.tv_usec = 100000;
  assert(select(fds[0] + 1, &rfds, NULL, NULL, &tv) == 1);
  assert(fd_isset_bit(fds[0], &rfds));

  assert(close(fds[0]) == 0);
  assert(close(fds[1]) == 0);
}

int main(void) {
  test_select();
  return 0;
}
