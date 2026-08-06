// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <signal.h>

void (*f3(int signum, void (*handler)(int)))(int) {
  return signal(signum, handler);
}
