// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <dlfcn.h>

void *f1(const char *filename, int flags) { return dlopen(filename, flags); }

void *f2(void *handle, const char *symbol) { return dlsym(handle, symbol); }

int f3(void *handle) { return dlclose(handle); }

char *f4(void) { return dlerror(); }
