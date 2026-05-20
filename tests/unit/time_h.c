// no-compile: refcount
#include <assert.h>
#include <stddef.h>
#include <stdio.h>
#include <string.h>
#include <sys/time.h>
#include <time.h>
#include <unistd.h>

static void test_time(void) { assert(time(NULL) > 0); }

static void test_gettimeofday(void) {
  struct timeval tv;
  tv.tv_sec = 0;
  tv.tv_usec = 0;
  assert(gettimeofday(&tv, NULL) == 0);
  assert(tv.tv_sec > 0);
  assert(tv.tv_usec >= 0);
  assert(tv.tv_usec < 1000000);
}

static void test_clock_gettime(void) {
  struct timespec ts;
  ts.tv_sec = 0;
  ts.tv_nsec = 0;
  assert(clock_gettime(CLOCK_MONOTONIC, &ts) == 0);
  assert(ts.tv_sec > 0);
  assert(ts.tv_nsec >= 0);
  assert(ts.tv_nsec < 1000000000);
}

static void test_localtime_r(void) {
  time_t t = 0;
  struct tm tm;
  assert(localtime_r(&t, &tm) != NULL);
  assert(tm.tm_year == 70);
}

static void test_gmtime_r(void) {
  time_t t = 0;
  struct tm tm;
  assert(gmtime_r(&t, &tm) != NULL);
  assert(tm.tm_year == 70);
  assert(tm.tm_mon == 0);
  assert(tm.tm_mday == 1);
  assert(tm.tm_hour == 0);
}

static void test_strftime(void) {
  time_t t = 0;
  struct tm tm;
  gmtime_r(&t, &tm);
  char buf[64];
  assert(strftime(buf, sizeof(buf), "%Y-%m-%d", &tm) == 10);
  assert(strcmp(buf, "1970-01-01") == 0);
}

static void test_utimes(void) {
  const char *path = "/tmp/cpp2rust_utimes_test.tmp";
  FILE *fp = fopen(path, "wb");
  assert(fp != NULL);
  assert(fclose(fp) == 0);
  struct timeval tv[2];
  tv[0].tv_sec = 1000;
  tv[0].tv_usec = 0;
  tv[1].tv_sec = 2000;
  tv[1].tv_usec = 0;
  assert(utimes(path, tv) == 0);
  unlink(path);
  assert(utimes(path, tv) == -1);
}

int main(void) {
  test_time();
  test_gettimeofday();
  test_clock_gettime();
  test_localtime_r();
  test_gmtime_r();
  test_strftime();
  test_utimes();
  return 0;
}
