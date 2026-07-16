#include <assert.h>
#include <errno.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/types.h>

static void test_send_recv(void) {
  int sv[2];
  assert(socketpair(AF_UNIX, SOCK_STREAM, 0, sv) == 0);
  const char *msg = "ping";
  assert(send(sv[0], msg, 4, 0) == 4);
  char buf[8] = {0};
  assert(recv(sv[1], buf, 8, 0) == 4);
  assert(memcmp(buf, "ping", 4) == 0);
  assert(send(sv[1], "pong!", 5, 0) == 5);
  assert(recv(sv[0], buf, 8, 0) == 5);
  assert(memcmp(buf, "pong!", 5) == 0);
}

static void test_send_recv_scalar(void) {
  int sv[2];
  assert(socketpair(AF_UNIX, SOCK_STREAM, 0, sv) == 0);
  int value = 42;
  assert(send(sv[0], &value, sizeof(value), 0) == sizeof(value));
  int received = 0;
  assert(recv(sv[1], &received, sizeof(received), 0) == sizeof(received));
  assert(received == 42);
}

static void test_send_bad_fd(void) {
  errno = 0;
  assert(send(-1, "x", 1, 0) == -1);
  assert(errno == EBADF);
}

int main(void) {
  test_send_recv();
  test_send_recv_scalar();
  test_send_bad_fd();
  return 0;
}
