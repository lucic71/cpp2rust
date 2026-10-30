// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#undef _GNU_SOURCE
#define _GNU_SOURCE
#include <unistd.h>

int f1(int fd) { return close(fd); }

off_t f2(int fd, off_t offset, int whence) { return lseek(fd, offset, whence); }

ssize_t f3(int fd, void *buf, size_t count) { return read(fd, buf, count); }

int f4(const char *pathname) { return unlink(pathname); }

int f5(int pipefd[2]) { return pipe(pipefd); }

int f6(int fd, off_t length) { return ftruncate(fd, length); }

int f7(int fd) { return isatty(fd); }

uid_t f8(void) { return geteuid(); }

int f9(char *name, size_t len) { return gethostname(name, len); }

ssize_t f10(int fd, const void *buf, size_t count) {
  return write(fd, buf, count);
}

int f11(const char *pathname) { return rmdir(pathname); }

size_t f12(int name, char *buf, size_t len) { return confstr(name, buf, len); }

int f13(const char *target, const char *linkpath) {
  return symlink(target, linkpath);
}

int f14(const char *oldpath, const char *newpath) {
  return link(oldpath, newpath);
}

char *f15(char *buf, size_t size) { return getcwd(buf, size); }

int f16(const char *path) { return chdir(path); }

pid_t f17(void) { return getpid(); }
