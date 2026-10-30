// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <sys/statvfs.h>

typedef struct statvfs t1;

int f1(const char *path, struct statvfs *buf) { return statvfs(path, buf); }
