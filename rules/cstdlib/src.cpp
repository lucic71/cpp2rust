// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstdlib>

void f1() { return std::abort(); }

void f2(void *a0) { return free(a0); }

void *f3(size_t a0) { return malloc(a0); }

void *f4(void *a0, size_t a1) { return realloc(a0, a1); }

void *f5(size_t nmemb, size_t size) { return calloc(nmemb, size); }

char *f6(const char *name) { return getenv(name); }

int f7(const char *name, const char *value, int overwrite) {
  return setenv(name, value, overwrite);
}

void *f8(const void *key, const void *base, size_t nmemb, size_t size,
         int (*compar)(const void *, const void *)) {
  return bsearch(key, base, nmemb, size, compar);
}

void f9(void *base, size_t nmemb, size_t size,
        int (*compar)(const void *, const void *)) {
  return qsort(base, nmemb, size, compar);
}

char *f10(const char *path, char *resolved_path) {
  return realpath(path, resolved_path);
}

void f11(int status) { return exit(status); }

int f12(const char *nptr) { return atoi(nptr); }

double f13(const char *nptr, char **endptr) { return strtod(nptr, endptr); }

long long f14(const char *nptr, char **endptr, int base) {
  return strtoll(nptr, endptr, base);
}

long f37(const char *nptr, char **endptr, int base) {
  return strtol(nptr, endptr, base);
}

unsigned long f38(const char *nptr, char **endptr, int base) {
  return strtoul(nptr, endptr, base);
}

int f39(char *tmpl) { return mkstemp(tmpl); }

char *f40(char *tmpl) { return mkdtemp(tmpl); }

int f41(const char *name) { return unsetenv(name); }
