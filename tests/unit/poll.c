// no-compile: refcount
#include <assert.h>
#include <poll.h>
#include <unistd.h>

static void test_poll_timeout(void) {
  int fds[2];
  assert(pipe(fds) == 0);
  struct pollfd pfd;
  pfd.fd = fds[0];
  pfd.events = POLLIN;
  pfd.revents = 0;
  assert(poll(&pfd, 1, 10) == 0);
  assert(pfd.revents == 0);
  assert(close(fds[0]) == 0);
  assert(close(fds[1]) == 0);
}

static void test_poll_readable(void) {
  int fds[2];
  assert(pipe(fds) == 0);
  assert(write(fds[1], "x", 1) == 1);
  struct pollfd pfd;
  pfd.fd = fds[0];
  pfd.events = POLLIN;
  pfd.revents = 0;
  assert(poll(&pfd, 1, 100) == 1);
  assert((pfd.revents & POLLIN) != 0);
  assert(close(fds[0]) == 0);
  assert(close(fds[1]) == 0);
}

static void test_poll_multiple(void) {
  int p1[2], p2[2];
  assert(pipe(p1) == 0);
  assert(pipe(p2) == 0);
  assert(write(p2[1], "y", 1) == 1);
  struct pollfd pfds[2];
  pfds[0].fd = p1[0];
  pfds[0].events = POLLIN;
  pfds[0].revents = 0;
  pfds[1].fd = p2[0];
  pfds[1].events = POLLIN;
  pfds[1].revents = 0;
  assert(poll(pfds, 2, 100) == 1);
  assert(pfds[0].revents == 0);
  assert((pfds[1].revents & POLLIN) != 0);
  assert(close(p1[0]) == 0);
  assert(close(p1[1]) == 0);
  assert(close(p2[0]) == 0);
  assert(close(p2[1]) == 0);
}

static void test_poll_hup(void) {
  int fds[2];
  assert(pipe(fds) == 0);
  assert(close(fds[1]) == 0);
  struct pollfd pfd;
  pfd.fd = fds[0];
  pfd.events = POLLIN;
  pfd.revents = 0;
  assert(poll(&pfd, 1, 100) >= 1);
  assert((pfd.revents & POLLHUP) != 0);
  assert(close(fds[0]) == 0);
}

int main(void) {
  test_poll_timeout();
  test_poll_readable();
  test_poll_multiple();
  test_poll_hup();
  return 0;
}
