// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

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

int f12(const char *pathname, uid_t owner, gid_t group) {
  return chown(pathname, owner, group);
}

int f13(const char *pathname, int mode) { return access(pathname, mode); }

ssize_t f14(const char *pathname, char *buf, size_t bufsiz) {
  return readlink(pathname, buf, bufsiz);
}

int f15(const char *target, const char *linkpath) {
  return symlink(target, linkpath);
}

char *f16(char *buf, size_t size) { return getcwd(buf, size); }

int f17(const char *path) { return chdir(path); }

int f18(int fd) { return fsync(fd); }

ssize_t f19(int fd, void *buf, size_t count, off_t offset) {
  return pread(fd, buf, count, offset);
}

ssize_t f20(int fd, const void *buf, size_t count, off_t offset) {
  return pwrite(fd, buf, count, offset);
}

pid_t f21(void) { return getpid(); }

uid_t f22(void) { return getuid(); }

int f23(int fd, uid_t owner, gid_t group) { return fchown(fd, owner, group); }

void f24(int status) { return _exit(status); }

int f25(const char *pathname, char *const argv[], char *const envp[]) {
  return execve(pathname, argv, envp);
}
