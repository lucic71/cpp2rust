// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <malloc.h>

size_t f1(void *ptr) { return malloc_usable_size(ptr); }
