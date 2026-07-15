// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <sys/ioctl.h>

template <typename... Args>
int f1(int a0, unsigned long a1, Args... args) {
  return ioctl(a0, a1, args...);
}
