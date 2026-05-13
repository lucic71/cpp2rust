// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstdlib>

void f1() { return std::abort(); }

void f2(void *a0) { return free(a0); }

void *f3(size_t a0) { return malloc(a0); }

void *f4(void *a0, size_t a1) { return realloc(a0, a1); }
