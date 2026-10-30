// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <sys/time.h>
#include <time.h>

typedef struct tm t1001;
typedef struct timeval t1002;
typedef struct timespec t1003;
typedef time_t t1004;

// struct timespec::tv_sec is `__time_t` on glibc and `__darwin_time_t` on
// Darwin. Give both time_t's translation.
//
// TODO:
//
//   Rules are written in terms of standard names `time_t tv_sec`, however
//   libraries choose to implement them using library specific names `__time_t
//   tv_sec`.
//
//   Fix this in converter by using the names in the rules, not the
//   library-specific names.

#if defined(__GLIBC__)
typedef __time_t t1005;
#elif defined(__APPLE__)
typedef __darwin_time_t t1005;
#else
#error "Unsupported platform for time_t"
#endif

time_t f1001(time_t *t) { return time(t); }

int f1002(clockid_t clk_id, struct timespec *tp) {
  return clock_gettime(clk_id, tp);
}

struct tm *f1004(const time_t *timer, struct tm *result) {
  return gmtime_r(timer, result);
}

struct tm *f1005(const time_t *timer, struct tm *result) {
  return localtime_r(timer, result);
}

size_t f1006(char *s, size_t maxsize, const char *format, const struct tm *tp) {
  return strftime(s, maxsize, format, tp);
}

int f1007(const char *file, const struct timeval tvp[2]) {
  return utimes(file, tvp);
}

#if defined(__linux__)
int f1008(struct timeval *tv, struct timezone *tz) { return gettimeofday(tv, tz); }
#elif defined(__APPLE__)
int f1008(struct timeval *tv, void *tz) { return gettimeofday(tv, tz); }
#else
#error "Unsupported platform for gettimeofday"
#endif

clockid_t f1009(void) { return CLOCK_REALTIME; }

clockid_t f1010(void) { return CLOCK_MONOTONIC; }

#ifdef __linux__
clockid_t f1011(void) { return CLOCK_MONOTONIC_RAW; }
#endif
typedef clock_t t1006;
