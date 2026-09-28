// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstddef>
#include <string>
#include <sys/types.h>

using t1 = std::byte;

typedef size_t t2;
typedef size_t *t3;
typedef const size_t *t4;
typedef std::string::size_type t5;
typedef std::string::size_type *t6;
typedef const std::string::size_type *t7;
typedef decltype(sizeof(0)) t8;
typedef decltype(sizeof(0)) *t9;
typedef const decltype(sizeof(0)) *t10;
typedef ssize_t t11;
typedef ssize_t *t12;
typedef const ssize_t *t13;

std::byte f1(const std::byte &a0, unsigned a1) { return operator<<(a0, a1); }

std::byte f2(const std::byte &a0, unsigned a1) { return operator>>(a0, a1); }

std::byte f3(std::byte &a0, unsigned a1) { return operator<<=(a0, a1); }

std::byte f4(std::byte &a0, unsigned a1) { return operator>>=(a0, a1); }
