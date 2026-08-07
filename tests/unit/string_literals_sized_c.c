#include <assert.h>

int main() {
  char empty_buf[256] = "";
  assert(empty_buf[0] == '\0');
  assert(empty_buf[255] == '\0');

  char prefix_buf[32] = "%";
  assert(prefix_buf[0] == '%');
  assert(prefix_buf[1] == '\0');
  assert(prefix_buf[31] == '\0');

  char short_buf[16] = "hi";
  assert(short_buf[0] == 'h');
  assert(short_buf[1] == 'i');
  assert(short_buf[2] == '\0');
  assert(short_buf[15] == '\0');

  char exact_buf[6] = "hello";
  assert(exact_buf[0] == 'h');
  assert(exact_buf[4] == 'o');
  assert(exact_buf[5] == '\0');

  assert(sizeof("hello") == 6);
  assert(sizeof("hello") - 1 == 5);
  assert(sizeof("") == 1);
  assert(sizeof("fifteen-bytes!!") - 1 == 15);

  char bytes[4];
  bytes[0] = 0xe2;
  bytes[1] = 0x90;
  bytes[2] = 0x80 + 1;
  bytes[3] = 0;
  assert(bytes[0] == (char)0xe2);
  assert((unsigned char)bytes[0] == 226);
  assert((unsigned char)bytes[1] == 144);
  assert((unsigned char)bytes[2] == 129);

  assert("Z"[0] == 'Z');
  assert("Z"[1] == 0);
  assert("ab"[2] == 0);
  assert("ab"[1] == 'b');

  int i = 1;
  assert("Z"[i] == 0);

  const char *p = "Z";
  assert(p[1] == 0);

  short wide = 0xffff;
  assert(wide == -1);
  unsigned char narrow = 300;
  assert(narrow == 44);
  return 0;
}
