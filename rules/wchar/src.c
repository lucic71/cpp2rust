// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <wchar.h>

typedef mbstate_t t1;
typedef wint_t t2;

wchar_t *f1(wchar_t *dst, const wchar_t *src) { return wcscpy(dst, src); }

#include <stdio.h>

wchar_t *f2(wchar_t *s, wchar_t c, size_t n) { return wmemset(s, c, n); }

int f3(const wchar_t *a, const wchar_t *b, size_t n) { return wcsncmp(a, b, n); }

int f4(const wchar_t *a, const wchar_t *b) { return wcscmp(a, b); }

int f5(const wchar_t *a, const wchar_t *b, size_t n) { return wmemcmp(a, b, n); }

size_t f6(const wchar_t *s) { return wcslen(s); }

long f7(const wchar_t *s, wchar_t **end, int base) { return wcstol(s, end, base); }

wint_t f8(FILE *stream) { return fgetwc(stream); }

wint_t f9(wint_t c, FILE *stream) { return ungetwc(c, stream); }

int f10(const wchar_t *s, FILE *stream) { return fputws(s, stream); }

int f11(FILE *stream, int mode) { return fwide(stream, mode); }
