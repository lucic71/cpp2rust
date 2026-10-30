// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <string.h>

#if defined(__linux__)
const void *f25(const void *a0, int a1, size_t a2) {
  return memrchr(a0, a1, a2);
}

void *f26(void *a0, int a1, size_t a2) { return memrchr(a0, a1, a2); }
#endif
