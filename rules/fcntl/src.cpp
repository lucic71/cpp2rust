// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <fcntl.h>

template <typename... Args>
int f1(int a0, int a1, Args... args) {
  return fcntl(a0, a1, args...);
}

template <typename... Args>
int f2(const char *a0, int a1, Args... args) {
  return open(a0, a1, args...);
}
