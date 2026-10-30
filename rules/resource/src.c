// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#undef _GNU_SOURCE
#define _GNU_SOURCE
#include <sys/resource.h>

typedef rlim_t t1;
typedef struct rlimit t2;

#ifdef __GLIBC__
typedef __rlim_t t3;
#endif

int f1(__rlimit_resource_t resource, struct rlimit *rlim) {
  return getrlimit(resource, rlim);
}

int f2(__rlimit_resource_t resource, const struct rlimit *rlim) {
  return setrlimit(resource, rlim);
}
