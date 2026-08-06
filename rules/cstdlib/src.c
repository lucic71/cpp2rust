// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <stdlib.h>

int f15(void (*function)(void)) { return atexit(function); }
