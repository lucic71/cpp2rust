#include <assert.h>
#include <sys/socket.h>
#include <sys/types.h>

int main() {
  assert(SOCK_STREAM == 1);
  assert(SOCK_DGRAM == 2);

  int x = SOCK_STREAM | SOCK_CLOEXEC;
  assert((x & SOCK_STREAM) == SOCK_STREAM);
  assert((x & SOCK_CLOEXEC) == SOCK_CLOEXEC);

  int y = SOCK_DGRAM | SOCK_NONBLOCK;
  assert((y & SOCK_DGRAM) == SOCK_DGRAM);
  assert((y & SOCK_NONBLOCK) == SOCK_NONBLOCK);

  return 0;
}
