// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <fcntl.h>

int (*f1)(int, int, ...) = fcntl;

int (*f2)(const char *, int, ...) = open;
