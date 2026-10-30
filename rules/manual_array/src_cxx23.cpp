// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <array>
#include <utility>

template <typename T1, std::size_t T2>
std::array<T1, T2> f4(std::array<T1, T2> &&o) {
  return std::array<T1, T2>(std::move(o));
}

template <typename T1, std::size_t T2>
std::array<T1, T2> &f5(std::array<T1, T2> &dst, std::array<T1, T2> &&src) {
  return dst = std::move(src);
}

template <typename T1, std::size_t T2>
std::array<T1, T2> f6(const std::array<T1, T2> &o) {
  return std::array<T1, T2>(o);
}

template <typename T1, std::size_t T2>
std::array<T1, T2> &f7(std::array<T1, T2> &dst, const std::array<T1, T2> &src) {
  return dst = src;
}
