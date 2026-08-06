// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <sys/resource.h>

typedef struct rusage t1;

int f1(int who, struct rusage *usage) { return getrusage(who, usage); }

int f2(void) { return RUSAGE_SELF; }

int f3(void) { return RUSAGE_CHILDREN; }
