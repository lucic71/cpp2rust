// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <sys/stat.h>

typedef struct stat t1;

int f1(const char *pathname, struct stat *statbuf) {
  return stat(pathname, statbuf);
}

int f2(int fd, struct stat *statbuf) { return fstat(fd, statbuf); }

int f3(const char *pathname, mode_t mode) { return mkdir(pathname, mode); }

int f4(const char *pathname, mode_t mode) { return chmod(pathname, mode); }

int f5(int dirfd, const char *pathname, const struct timespec *times,
       int flags) {
  return utimensat(dirfd, pathname, times, flags);
}

int f6(const char *pathname, struct stat *statbuf) {
  return lstat(pathname, statbuf);
}

int f7(int fd, mode_t mode) { return fchmod(fd, mode); }
