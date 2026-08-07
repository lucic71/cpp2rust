// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <sys/wait.h>

pid_t f1(pid_t pid, int *wstatus, int options) {
  return waitpid(pid, wstatus, options);
}

int f2(void) { return WNOHANG; }

int f3(void) { return WUNTRACED; }
