#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

int main(void) {
  const char *path = "cpp2rust_read_ahead.tmp";
  char buf[64];

  FILE *fp = fopen(path, "w");
  assert(fp != NULL);
  fputs("line1\nline2\nline3\n", fp);
  fclose(fp);

  assert(freopen(path, "r", stdin) != NULL);
  assert(fgets(buf, sizeof(buf), stdin) != NULL);
  assert(strcmp(buf, "line1\n") == 0);

  FILE *pipe = popen("cat", "r");
  assert(pipe != NULL);
  size_t n = fread(buf, 1, sizeof(buf) - 1, pipe);
  assert(pclose(pipe) == 0);
  assert(n == 0);

  fp = fopen(path, "r");
  assert(fp != NULL);
  assert(fgetc(fp) == 'l');
  assert(ftell(fp) == 1);
  assert(fseek(fp, 5, SEEK_CUR) == 0);
  assert(ftell(fp) == 6);
  assert(fgetc(fp) == 'l');
  assert(ftell(fp) == 7);
  fclose(fp);

  assert(unlink(path) == 0);
  return 0;
}
