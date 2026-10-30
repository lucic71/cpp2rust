// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <utility>

template <typename T1, typename T2> const T2 &f15(const std::pair<T1, T2> &a0) {
  return a0.second;
}

template <typename T1, typename T2> const T1 &f16(const std::pair<T1, T2> &a0) {
  return a0.first;
}
