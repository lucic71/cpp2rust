// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <cstddef>

#if __cplusplus >= 201703L
using t1 = std::byte;
#endif

using t2 = std::size_t;
using t3 = std::size_t *;
using t4 = const std::size_t *;

#if __cplusplus >= 201703L
std::byte f1(const std::byte &a0, unsigned a1) { return operator<<(a0, a1); }

std::byte f2(const std::byte &a0, unsigned a1) { return operator>>(a0, a1); }

std::byte f3(std::byte &a0, unsigned a1) { return operator<<=(a0, a1); }

std::byte f4(std::byte &a0, unsigned a1) { return operator>>=(a0, a1); }
#endif
