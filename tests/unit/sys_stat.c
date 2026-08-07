#include <assert.h>
#include <errno.h>
#include <stdio.h>
#include <sys/stat.h>
#include <unistd.h>

static void test_stat(void) {
  const char *path = "cpp2rust_stat_test.tmp";
  FILE *fp = fopen(path, "wb");
  assert(fp != NULL);
  fputs("hello", fp);
  assert(fclose(fp) == 0);
  struct stat st;
  assert(stat(path, &st) == 0);
  assert(st.st_size == 5);
  assert(st.st_mtime > 0);
  unlink(path);
}

static void test_fstat(void) {
  const char *path = "cpp2rust_fstat_test.tmp";
  FILE *fp = fopen(path, "wb");
  assert(fp != NULL);
  fputs("hello world", fp);
  fflush(fp);
  int fd = fileno(fp);
  struct stat st;
  assert(fstat(fd, &st) == 0);
  assert(st.st_size == 11);
  assert(st.st_mtime > 0);
  assert(fclose(fp) == 0);
  unlink(path);
}

static long long timespec_to_ms(const struct timespec *tv) {
  return (long long)tv->tv_sec * 1000 + (tv->tv_nsec / 1000000);
}

static void test_timespec_members(void) {
  const char *path = "cpp2rust_stat_ts_test.tmp";
  FILE *fp = fopen(path, "wb");
  assert(fp != NULL);
  fputs("hi", fp);
  assert(fclose(fp) == 0);

  struct stat st;
  assert(stat(path, &st) == 0);

  assert(timespec_to_ms(&st.st_atim) >= st.st_atime * 1000);
  assert(timespec_to_ms(&st.st_mtim) >= st.st_mtime * 1000);
  assert(timespec_to_ms(&st.st_ctim) >= st.st_ctime * 1000);

  assert(st.st_mtim.tv_sec == st.st_mtime);
  assert(st.st_mtim.tv_nsec >= 0);

  struct timespec copy = st.st_mtim;
  assert(copy.tv_sec == st.st_mtime);

  unlink(path);
}

int main(void) {
  test_stat();
  test_fstat();
  test_timespec_members();
  return 0;
}
