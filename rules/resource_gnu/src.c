// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

// Same rules as rules/resource, but compiled with _GNU_SOURCE so that the
// recorded signatures use glibc's enum parameter types (__rusage_who,
// __rlimit_resource) instead of int.
#define _GNU_SOURCE
#include <sys/resource.h>

int f1(int who, struct rusage *usage) { return getrusage(who, usage); }

int f2(int resource, struct rlimit *rlim) { return getrlimit(resource, rlim); }

int f3(int resource, const struct rlimit *rlim) {
  return setrlimit(resource, rlim);
}
