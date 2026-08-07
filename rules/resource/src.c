// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <sys/resource.h>

typedef struct rusage t1;

typedef enum __rusage_who t2;

typedef enum __rlimit_resource t3;

typedef enum __priority_which t4;

typedef struct rlimit t5;

int f1(int who, struct rusage *usage) { return getrusage(who, usage); }

int f2(void) { return RUSAGE_SELF; }

int f3(void) { return RUSAGE_CHILDREN; }

int f4(int resource, struct rlimit *rlim) { return getrlimit(resource, rlim); }

int f5(int resource, const struct rlimit *rlim) {
  return setrlimit(resource, rlim);
}

int f6(void) { return RLIMIT_STACK; }

int f7(void) { return RLIMIT_DATA; }

int f8(void) { return RLIMIT_NOFILE; }

int f9(void) { return RLIMIT_CORE; }
