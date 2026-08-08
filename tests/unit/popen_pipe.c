#include <assert.h>
#include <stdio.h>
#include <string.h>

int main(void) {
  char buf[64];

  FILE *in = popen("echo hello", "r");
  assert(in != NULL);
  assert(fgets(buf, sizeof(buf), in) != NULL);
  assert(pclose(in) == 0);
  assert(strcmp(buf, "hello\n") == 0);

  FILE *out = popen("cat > /dev/null", "w");
  assert(out != NULL);
  assert(fputs("data\n", out) >= 0);
  assert(pclose(out) == 0);

  printf("%s", buf);
  return 0;
}
