// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <deque>
#include <vector>

template <typename T1> using t1 = std::deque<T1>;

template <typename T1> using t2 = typename std::deque<T1>::size_type;

template <typename T1> using t3 = typename std::deque<T1>::value_type;

template <typename T1> using t4 = typename std::deque<T1>::reference;

template <typename T1> using t5 = typename std::deque<T1>::const_reference;

template <typename T1> T1 &f1(std::deque<T1> &o) { return o.back(); }

template <typename T1> T1 &f2(std::deque<T1> &o) { return o.front(); }

template <typename T1> bool f3(const std::deque<T1> &o) { return o.empty(); }

template <typename T1> void f4(std::deque<T1> &o, T1 &&value) {
  return o.push_back(std::move(value));
}

template <typename T1> void f5(std::deque<T1> &o) { return o.pop_front(); }

template <typename T1> void f7(std::deque<T1> &o, const T1 &value) {
  return o.push_back(value);
}

template <typename T1> std::deque<T1> f8(const std::deque<T1> &o) {
  return std::deque<T1>(o);
}

template <typename T1> std::deque<T1> f9(std::deque<T1> &&o) {
  return std::deque<T1>(std::move(o));
}

template <typename T1>
std::deque<T1> &f10(std::deque<T1> &dst, const std::deque<T1> &src) {
  return dst = src;
}

template <typename T1>
std::deque<T1> &f11(std::deque<T1> &dst, std::deque<T1> &&src) {
  return dst = std::move(src);
}

template <typename T1, typename... T2>
auto f12(std::deque<T1> &o, T2 &&...a1)
    -> decltype(o.emplace_back(std::forward<T2>(a1)...)) {
  return o.emplace_back(std::forward<T2>(a1)...);
}

template <typename T1> std::deque<T1> f14() { return std::deque<T1>(); }

template <typename T1> void f15(std::deque<T1> &o, const T1 &value) {
  return o.push_front(value);
}

template <typename T1> void f16(std::deque<T1> &o, T1 &&value) {
  return o.push_front(std::move(value));
}

template <typename T1, typename... T2>
auto f17(std::deque<T1> &o, T2 &&...a1)
    -> decltype(o.emplace_front(std::forward<T2>(a1)...)) {
  return o.emplace_front(std::forward<T2>(a1)...);
}

template <typename T1> void f18(std::deque<T1> &o) { return o.pop_back(); }

template <typename T1> std::size_t f19(const std::deque<T1> &o) {
  return o.size();
}

template <typename T1> T1 &f20(std::deque<T1> &o, std::size_t idx) {
  return o.operator[](idx);
}

template <typename T1>
const T1 &f21(const std::deque<T1> &o, std::size_t idx) {
  return o.operator[](idx);
}

template <typename T1> T1 &f22(std::deque<T1> &o, std::size_t idx) {
  return o.at(idx);
}

template <typename T1>
const T1 &f23(const std::deque<T1> &o, std::size_t idx) {
  return o.at(idx);
}

template <typename T1> const T1 &f24(const std::deque<T1> &o) {
  return o.front();
}

template <typename T1> const T1 &f25(const std::deque<T1> &o) {
  return o.back();
}

template <typename T1>
typename std::deque<T1>::iterator f26(std::deque<T1> &o) {
  return o.begin();
}

template <typename T1>
typename std::deque<T1>::iterator f27(std::deque<T1> &o) {
  return o.end();
}

template <typename T1>
typename std::deque<T1>::const_iterator f28(const std::deque<T1> &o) {
  return o.begin();
}

template <typename T1>
typename std::deque<T1>::const_iterator f29(const std::deque<T1> &o) {
  return o.end();
}

template <typename T1> void f30(std::deque<T1> &o) { return o.clear(); }

template <typename T1>
typename std::deque<T1>::iterator
f31(std::deque<T1> &o, typename std::deque<T1>::const_iterator it,
    const T1 &value) {
  return o.insert(it, value);
}

template <typename T1>
typename std::deque<T1>::iterator
f32(std::deque<T1> &o, typename std::deque<T1>::const_iterator it,
    T1 &&value) {
  return o.insert(it, std::move(value));
}

template <typename T1>
typename std::deque<T1>::iterator
f33(std::deque<T1> &o, typename std::deque<T1>::const_iterator it) {
  return o.erase(it);
}

template <typename T1> std::deque<T1> f34(std::size_t n) {
  return std::deque<T1>(n);
}

template <typename T1> std::deque<T1> f35(std::size_t n, const T1 &value) {
  return std::deque<T1>(n, value);
}
