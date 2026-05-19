// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <sys/time.h>
#include <time.h>

time_t f1(time_t *tloc) { return time(tloc); }

int f2(struct timeval *tv, struct timezone *tz) {
  return gettimeofday(tv, tz);
}

int f3(clockid_t clk_id, struct timespec *tp) { return clock_gettime(clk_id, tp); }

struct tm *f4(const time_t *timep, struct tm *result) {
  return localtime_r(timep, result);
}

struct tm *f5(const time_t *timep, struct tm *result) {
  return gmtime_r(timep, result);
}

size_t f6(char *s, size_t max, const char *fmt, const struct tm *tm) {
  return strftime(s, max, fmt, tm);
}

int f7(const char *filename, const struct timeval times[2]) {
  return utimes(filename, times);
}
