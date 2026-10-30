// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#undef _GNU_SOURCE
#define _GNU_SOURCE
#include <stdlib.h>

void f1001(void) { return abort(); }

void f1002(void *a0) { return free(a0); }

void *f1003(size_t a0) { return malloc(a0); }

void *f1004(void *a0, size_t a1) { return realloc(a0, a1); }

void *f1005(size_t nmemb, size_t size) { return calloc(nmemb, size); }

char *f1006(const char *name) { return getenv(name); }

int f1007(const char *name, const char *value, int overwrite) {
  return setenv(name, value, overwrite);
}

void *f1008(const void *key, const void *base, size_t nmemb, size_t size,
         int (*compar)(const void *, const void *)) {
  return bsearch(key, base, nmemb, size, compar);
}

void f1009(void *base, size_t nmemb, size_t size,
        int (*compar)(const void *, const void *)) {
  return qsort(base, nmemb, size, compar);
}

char *f1010(const char *path, char *resolved_path) {
  return realpath(path, resolved_path);
}

int f1011(char *a0) { return mkstemp(a0); }

int f1012(const char *a0) { return system(a0); }

void f1013(int a0) { return exit(a0); }

void f1014(int a0) { return quick_exit(a0); }

int f1015(void (*a0)(void)) { return at_quick_exit(a0); }

int f1016(int a0) { return abs(a0); }

long f1017(long a0) { return labs(a0); }

long long f1018(long long a0) { return llabs(a0); }

int f1021(void) { return rand(); }

void f1022(unsigned a0) { return srand(a0); }

long f1023(const char *a0, char **a1, int a2) { return strtol(a0, a1, a2); }
