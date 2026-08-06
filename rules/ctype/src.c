// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <ctype.h>

int f1(int c) { return cpp2rust_isalnum(c); }

int f2(int c) { return cpp2rust_isalpha(c); }

int f3(int c) { return cpp2rust_isblank(c); }

int f4(int c) { return cpp2rust_iscntrl(c); }

int f5(int c) { return cpp2rust_isdigit(c); }

int f6(int c) { return cpp2rust_isgraph(c); }

int f7(int c) { return cpp2rust_islower(c); }

int f8(int c) { return cpp2rust_isprint(c); }

int f9(int c) { return cpp2rust_ispunct(c); }

int f10(int c) { return cpp2rust_isspace(c); }

int f11(int c) { return cpp2rust_isupper(c); }

int f12(int c) { return cpp2rust_isxdigit(c); }

int f13(int c) { return cpp2rust_tolower(c); }

int f14(int c) { return cpp2rust_toupper(c); }
