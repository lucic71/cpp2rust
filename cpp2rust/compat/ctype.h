// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include_next <ctype.h>

#ifndef __cplusplus

#undef isalnum
#undef isalpha
#undef isblank
#undef iscntrl
#undef isdigit
#undef isgraph
#undef islower
#undef isprint
#undef ispunct
#undef isspace
#undef isupper
#undef isxdigit
#undef tolower
#undef toupper

int cpp2rust_isalnum(int c);
int cpp2rust_isalpha(int c);
int cpp2rust_isblank(int c);
int cpp2rust_iscntrl(int c);
int cpp2rust_isdigit(int c);
int cpp2rust_isgraph(int c);
int cpp2rust_islower(int c);
int cpp2rust_isprint(int c);
int cpp2rust_ispunct(int c);
int cpp2rust_isspace(int c);
int cpp2rust_isupper(int c);
int cpp2rust_isxdigit(int c);
int cpp2rust_tolower(int c);
int cpp2rust_toupper(int c);

#define isalnum(c) cpp2rust_isalnum(c)
#define isalpha(c) cpp2rust_isalpha(c)
#define isblank(c) cpp2rust_isblank(c)
#define iscntrl(c) cpp2rust_iscntrl(c)
#define isdigit(c) cpp2rust_isdigit(c)
#define isgraph(c) cpp2rust_isgraph(c)
#define islower(c) cpp2rust_islower(c)
#define isprint(c) cpp2rust_isprint(c)
#define ispunct(c) cpp2rust_ispunct(c)
#define isspace(c) cpp2rust_isspace(c)
#define isupper(c) cpp2rust_isupper(c)
#define isxdigit(c) cpp2rust_isxdigit(c)
#define tolower(c) cpp2rust_tolower(c)
#define toupper(c) cpp2rust_toupper(c)

#endif
