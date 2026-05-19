// no-compile: refcount
#include <assert.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <unistd.h>

static void test_socket(void) {
  int s = socket(AF_INET, SOCK_STREAM, 0);
  assert(s >= 0);
  assert(close(s) == 0);
}

static void test_setsockopt_getsockopt(void) {
  int s = socket(AF_INET, SOCK_STREAM, 0);
  assert(s >= 0);
  int set = 1;
  assert(setsockopt(s, SOL_SOCKET, SO_REUSEADDR, &set, sizeof(set)) == 0);
  int got = 0;
  socklen_t len = sizeof(got);
  assert(getsockopt(s, SOL_SOCKET, SO_REUSEADDR, &got, &len) == 0);
  assert(got != 0);
  assert(close(s) == 0);
}

static void test_send_recv(void) {
  int fds[2];
  assert(socketpair(AF_UNIX, SOCK_STREAM, 0, fds) == 0);
  const char *msg = "hello";
  assert(send(fds[0], msg, 5, 0) == 5);
  char buf[8] = {0};
  assert(recv(fds[1], buf, 8, 0) == 5);
  assert(memcmp(buf, msg, 5) == 0);
  assert(close(fds[0]) == 0);
  assert(close(fds[1]) == 0);
}

int main(void) {
  test_socket();
  test_setsockopt_getsockopt();
  test_send_recv();
  return 0;
}
