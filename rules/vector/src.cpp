// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <algorithm>
#include <initializer_list>
#include <iterator>
#include <type_traits>
#include <vector>

template <typename T, typename A> using Init = A;

template <typename T1> using t1 = std::vector<T1>;
template <
    typename It, typename T1 = typename std::iterator_traits<It>::value_type,
    typename = typename std::enable_if<
        std::is_same<It, typename std::vector<T1>::iterator>::value>::type>
void t2(It *);
template <typename T1> using t3 = std::vector<std::vector<T1>>;
template <typename It,

          typename T1 = typename std::iterator_traits<It>::value_type,
          typename = typename std::enable_if<std::is_same<
              It, typename std::vector<T1>::const_iterator>::value>::type>
void t4(It *);

template <typename T1, typename T2 = std::allocator<T1>>
using t5 = std::vector<T1, T2>;

#if defined(__linux__)
template <template <typename...> class Iterator, typename Pointer, typename T1,
          typename T2,
          typename = typename std::enable_if<std::is_same<
              Iterator<Pointer, std::vector<T1, T2>>,
              typename std::vector<T1, T2>::iterator>::value>::type>
void t6(Iterator<Pointer, std::vector<T1, T2>> *);
template <template <typename...> class Iterator, typename Pointer, typename T1,
          typename T2,
          typename = typename std::enable_if<std::is_same<
              Iterator<Pointer, std::vector<T1, T2>>,
              typename std::vector<T1, T2>::const_iterator>::value>::type>
void t7(Iterator<Pointer, std::vector<T1, T2>> *);
#endif

template <typename T1>
typename std::vector<T1>::iterator
f1(std::vector<T1> &o, typename std::vector<T1>::const_iterator it) {
  return o.erase(it);
}

template <typename T1> std::size_t f2(const std::vector<T1> &o) {
  return o.size();
}
template <typename T1> bool f3(const std::vector<T1> &o) { return o.empty(); }

template <typename T1> std::vector<T1> f4() { return std::vector<T1>(); }

template <typename T1> void f5(std::vector<T1> &o) { return o.pop_back(); }

template <typename T1> T1 *f6(std::vector<T1> &o) { return o.data(); }

template <typename T1> T1 &f7(std::vector<T1> &o, std::size_t idx) {
  return o.at(idx);
}

template <typename T1> std::vector<T1> f8(std::size_t n) {
  return std::vector<T1>(n);
}

template <typename T1> T1 &f9(std::vector<T1> &o) { return o.front(); }

template <typename T1> T1 &f10(std::vector<T1> &o) { return o.back(); }

template <typename T1> std::size_t f11(const std::vector<T1> &o) {
  return o.capacity();
}

template <typename T1> void f12(std::vector<T1> &o, std::size_t n) {
  return o.reserve(n);
}

template <typename T1>
typename std::vector<T1>::iterator f13(std::vector<T1> &o) {
  return o.begin();
}

template <typename T1> void f14(std::vector<T1> &o, T1 &&value) {
  return o.push_back(std::move(value));
}

template <typename T1> void f15(std::vector<T1> &o, std::size_t n) {
  return o.resize(n);
}

template <typename T1> void f16(std::vector<T1> &o) { return o.clear(); }

template <typename T1>
typename std::vector<T1>::iterator f17(std::vector<T1> &o) {
  return o.end();
}

template <typename T1>
typename std::vector<T1>::iterator
f18(std::vector<T1> &o, typename std::vector<T1>::const_iterator it,
    T1 &&value) {
  return o.insert(it, std::move(value));
}

template <typename T1> std::vector<T1> f19(std::size_t n, const T1 &value) {
  return std::vector<T1>(n, value);
}

template <typename T1>
typename std::vector<T1>::iterator
f20(std::vector<T1> &o, typename std::vector<T1>::const_iterator it,
    const T1 &value) {
  return o.insert(it, value);
}

template <typename T1> void f21(std::vector<T1> &o, const T1 &value) {
  return o.push_back(value);
}

template <
    typename It, typename T1 = typename std::iterator_traits<It>::value_type,
    typename = typename std::enable_if<
        std::is_same<It, typename std::vector<T1>::iterator>::value>::type>
typename std::vector<T1>::reference f22(It it) {
  return it.operator*();
}

template <
    typename It, typename T1 = typename std::iterator_traits<It>::value_type,
    typename = typename std::enable_if<
        std::is_same<It, typename std::vector<T1>::iterator>::value>::type>
typename std::vector<T1>::iterator f23(const It &it) {
  return typename std::vector<T1>::iterator(it);
}

template <
    typename It, typename T1 = typename std::iterator_traits<It>::value_type,
    typename = typename std::enable_if<
        std::is_same<It, typename std::vector<T1>::iterator>::value>::type>
typename std::vector<T1>::const_iterator f24(const It &it) {
  return typename std::vector<T1>::const_iterator(it);
}

template <
    typename It, typename T1 = typename std::iterator_traits<It>::value_type,
    typename = typename std::enable_if<
        std::is_same<It, typename std::vector<T1>::iterator>::value>::type>
typename std::vector<T1>::iterator f25(It it, std::size_t n) {
  return it.operator+(n);
}

template <
    typename It, typename T1 = typename std::iterator_traits<It>::value_type,
    typename = typename std::enable_if<
        std::is_same<It, typename std::vector<T1>::iterator>::value>::type>
bool f26(const It &it1, const It &it2) {
  return operator!=(it1, it2);
}

template <
    typename It, typename T1 = typename std::iterator_traits<It>::value_type,
    typename = typename std::enable_if<
        std::is_same<It, typename std::vector<T1>::iterator>::value>::type>
bool f27(const It &it1, const It &it2) {
  return operator==(it1, it2);
}

template <
    typename It, typename T1 = typename std::iterator_traits<It>::value_type,
    typename = typename std::enable_if<
        std::is_same<It, typename std::vector<T1>::iterator>::value>::type>
typename std::vector<T1>::iterator f28(It a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
std::vector<std::vector<T1>> f29(const std::vector<std::vector<T1>> &&o) {
  return std::vector<std::vector<T1>>(std::move(o));
}

template <typename T1> std::vector<std::vector<T1>> f30(std::size_t n) {
  return std::vector<std::vector<T1>>(n);
}

template <typename T1>
void f31(std::vector<std::vector<T1>> &o, std::vector<T1> &&value) {
  return o.push_back(std::move(value));
}

template <typename T1>
void f32(std::vector<std::vector<T1>> &o, std::size_t n) {
  return o.resize(n);
}

template <
    typename It, typename T1 = typename std::iterator_traits<It>::value_type,
    typename = typename std::enable_if<
        std::is_same<It, typename std::vector<T1>::iterator>::value>::type>
typename std::vector<T1>::iterator::difference_type f33(const It &it1,
                                                        const It &it2) {
  return operator-(it1, it2);
}

template <
    typename It, typename T1 = typename std::iterator_traits<It>::value_type,
    typename = typename std::enable_if<
        std::is_same<It, typename std::vector<T1>::iterator>::value>::type>
typename std::vector<T1>::iterator &f34(It &it) {
  return it.operator++();
}

template <typename T1> std::vector<T1> f35(const T1 *first, const T1 *last) {
  return std::vector<T1>(first, last);
}

template <typename T1>
std::vector<T1> f36(const std::initializer_list<T1> &a0) {
  return std::vector<T1>(a0);
}

template <typename T1, typename T2> std::vector<T1> f37(T2 *first, T2 *last) {
  return std::vector<T1>(first, last);
}

std::vector<bool> f38(std::size_t n, const bool &value) {
  return std::vector<bool>(n, value);
}

template <class T1, std::size_t T2> const T1 *f40(T1 const (&a0)[T2]) {
  return std::end(a0);
}

template <typename T1> const T1 *f41(const std::vector<T1> &o) {
  return o.data();
}

template <typename It,

          typename T1 = typename std::iterator_traits<It>::value_type,
          typename = typename std::enable_if<std::is_same<
              It, typename std::vector<T1>::const_iterator>::value>::type>
typename std::vector<T1>::const_iterator f42(It first, It last) {
  return std::max_element(first, last);
}

template <typename T1>
typename std::vector<T1>::const_iterator f43(const std::vector<T1> &o) {
  return o.begin();
}

template <typename T1>
typename std::vector<T1>::const_iterator f44(const std::vector<T1> &o) {
  return o.end();
}

bool f47(std::vector<bool> &o) { return o[0]; }

template <typename T1> void f48(std::vector<T1> &o, std::vector<T1> &a0) {
  return o.swap(a0);
}

template <typename T1>
const T1 &f50(const std::vector<T1> &o, std::size_t idx) {
  return o.at(idx);
}

template <typename T1> const T1 &f51(const std::vector<T1> &o) {
  return o.back();
}

template <typename T1>
void f52(std::vector<std::vector<T1>> &o, const std::vector<T1> &value) {
  return o.push_back(value);
}

template <typename T1>
typename std::vector<T1>::iterator
f53(std::vector<T1> &o, typename std::vector<T1>::const_iterator pos,
    const T1 *first, const T1 *last) {
  return o.insert(pos, first, last);
}

template <typename T1>
void f54(std::vector<T1> &o, std::size_t n,
         const typename std::vector<T1>::value_type &value) {
  return o.resize(n, value);
}

template <typename T1>
std::vector<T1> &f55(std::vector<T1> &dst, std::vector<T1> &&src) {
  return dst.operator=(std::move(src));
}

template <typename T1> std::vector<T1> &f56(std::vector<std::vector<T1>> &o) {
  return o.back();
}

template <typename T1>
typename std::vector<T1>::const_iterator f57(const std::vector<T1> &o) {
  return o.cend();
}

template <typename T1>
std::vector<T1> &f58(std::vector<T1> &dst, const std::vector<T1> &src) {
  return dst.operator=(src);
}

template <typename T1> void f59(std::vector<T1> &o) {
  return o.shrink_to_fit();
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::iterator
f60(std::vector<T1, T2> &o, typename std::vector<T1, T2>::const_iterator it) {
  return o.erase(it);
}

template <typename T1, typename T2 = std::allocator<T1>>
std::size_t f61(const std::vector<T1, T2> &o) {
  return o.size();
}

template <typename T1, typename T2 = std::allocator<T1>>
bool f62(const std::vector<T1, T2> &o) {
  return o.empty();
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> f63() {
  return std::vector<T1, T2>();
}

template <typename T1, typename T2 = std::allocator<T1>>
void f64(std::vector<T1, T2> &o) {
  return o.pop_back();
}

template <typename T1, typename T2 = std::allocator<T1>>
T1 *f65(std::vector<T1, T2> &o) {
  return o.data();
}

template <typename T1, typename T2 = std::allocator<T1>>
T1 &f66(std::vector<T1, T2> &o, std::size_t idx) {
  return o.at(idx);
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> f67(std::size_t n) {
  return std::vector<T1, T2>(n);
}

template <typename T1, typename T2 = std::allocator<T1>>
T1 &f68(std::vector<T1, T2> &o) {
  return o.front();
}

template <typename T1, typename T2 = std::allocator<T1>>
T1 &f69(std::vector<T1, T2> &o) {
  return o.back();
}

template <typename T1, typename T2 = std::allocator<T1>>
std::size_t f70(const std::vector<T1, T2> &o) {
  return o.capacity();
}

template <typename T1, typename T2 = std::allocator<T1>>
void f71(std::vector<T1, T2> &o, std::size_t n) {
  return o.reserve(n);
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::iterator f72(std::vector<T1, T2> &o) {
  return o.begin();
}

template <typename T1, typename T2 = std::allocator<T1>>
void f73(std::vector<T1, T2> &o, T1 &&value) {
  return o.push_back(std::move(value));
}

template <typename T1, typename T2 = std::allocator<T1>>
void f74(std::vector<T1, T2> &o, std::size_t n) {
  return o.resize(n);
}

template <typename T1, typename T2 = std::allocator<T1>>
void f75(std::vector<T1, T2> &o) {
  return o.clear();
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::iterator f76(std::vector<T1, T2> &o) {
  return o.end();
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::iterator
f77(std::vector<T1, T2> &o, typename std::vector<T1, T2>::const_iterator it,
    T1 &&value) {
  return o.insert(it, std::move(value));
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> f78(std::size_t n, const T1 &value) {
  return std::vector<T1, T2>(n, value);
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::iterator
f79(std::vector<T1, T2> &o, typename std::vector<T1, T2>::const_iterator it,
    const T1 &value) {
  return o.insert(it, value);
}

template <typename T1, typename T2 = std::allocator<T1>>
void f80(std::vector<T1, T2> &o, const T1 &value) {
  return o.push_back(value);
}

template <template <typename...> class Iterator, typename Pointer, typename T1,
          typename T2,
          typename = typename std::enable_if<std::is_same<
              Iterator<Pointer, std::vector<T1, T2>>,
              typename std::vector<T1, T2>::iterator>::value>::type>
typename std::vector<T1, T2>::reference
f81(Iterator<Pointer, std::vector<T1, T2>> it) {
  return it.operator*();
}

template <template <typename...> class Iterator, typename Pointer, typename T1,
          typename T2,
          typename = typename std::enable_if<std::is_same<
              Iterator<Pointer, std::vector<T1, T2>>,
              typename std::vector<T1, T2>::iterator>::value>::type>
typename std::vector<T1, T2>::iterator
f82(const Iterator<Pointer, std::vector<T1, T2>> &it) {
  return typename std::vector<T1, T2>::iterator(it);
}

template <template <typename...> class Iterator, typename Pointer, typename T1,
          typename T2,
          typename = typename std::enable_if<std::is_same<
              Iterator<Pointer, std::vector<T1, T2>>,
              typename std::vector<T1, T2>::iterator>::value>::type>
typename std::vector<T1, T2>::const_iterator
f83(const Iterator<Pointer, std::vector<T1, T2>> &it) {
  return typename std::vector<T1, T2>::const_iterator(it);
}

template <template <typename...> class Iterator, typename Pointer, typename T1,
          typename T2,
          typename = typename std::enable_if<std::is_same<
              Iterator<Pointer, std::vector<T1, T2>>,
              typename std::vector<T1, T2>::iterator>::value>::type>
typename std::vector<T1, T2>::iterator
f84(Iterator<Pointer, std::vector<T1, T2>> it, std::size_t n) {
  return it.operator+(n);
}

template <template <typename...> class Iterator, typename Pointer, typename T1,
          typename T2,
          typename = typename std::enable_if<std::is_same<
              Iterator<Pointer, std::vector<T1, T2>>,
              typename std::vector<T1, T2>::iterator>::value>::type>
bool f85(const Iterator<Pointer, std::vector<T1, T2>> &it1,
         const Iterator<Pointer, std::vector<T1, T2>> &it2) {
  return operator!=(it1, it2);
}

template <template <typename...> class Iterator, typename Pointer, typename T1,
          typename T2,
          typename = typename std::enable_if<std::is_same<
              Iterator<Pointer, std::vector<T1, T2>>,
              typename std::vector<T1, T2>::iterator>::value>::type>
bool f86(const Iterator<Pointer, std::vector<T1, T2>> &it1,
         const Iterator<Pointer, std::vector<T1, T2>> &it2) {
  return operator==(it1, it2);
}

template <template <typename...> class Iterator, typename Pointer, typename T1,
          typename T2,
          typename = typename std::enable_if<std::is_same<
              Iterator<Pointer, std::vector<T1, T2>>,
              typename std::vector<T1, T2>::iterator>::value>::type>
typename std::vector<T1, T2>::iterator
f87(Iterator<Pointer, std::vector<T1, T2>> a0, int a1) {
  return a0.operator++(a1);
}

template <template <typename...> class Iterator, typename Pointer, typename T1,
          typename T2,
          typename = typename std::enable_if<std::is_same<
              Iterator<Pointer, std::vector<T1, T2>>,
              typename std::vector<T1, T2>::iterator>::value>::type>
typename std::vector<T1, T2>::iterator::difference_type
f88(const Iterator<Pointer, std::vector<T1, T2>> &it1,
    const Iterator<Pointer, std::vector<T1, T2>> &it2) {
  return operator-(it1, it2);
}

template <template <typename...> class Iterator, typename Pointer, typename T1,
          typename T2,
          typename = typename std::enable_if<std::is_same<
              Iterator<Pointer, std::vector<T1, T2>>,
              typename std::vector<T1, T2>::iterator>::value>::type>
typename std::vector<T1, T2>::iterator &
f89(Iterator<Pointer, std::vector<T1, T2>> &it) {
  return it.operator++();
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> f90(const T1 *first, const T1 *last) {
  return std::vector<T1, T2>(first, last);
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> f91(const std::initializer_list<T1> &a0) {
  return std::vector<T1, T2>(a0);
}

template <typename T1, typename T2 = std::allocator<T1>, typename T3>
std::vector<T1, T2> f92(T3 *first, T3 *last) {
  return std::vector<T1, T2>(first, last);
}

template <typename T1, typename T2 = std::allocator<T1>>
const T1 *f93(const std::vector<T1, T2> &o) {
  return o.data();
}

template <template <typename...> class Iterator, typename Pointer, typename T1,
          typename T2,
          typename = typename std::enable_if<std::is_same<
              Iterator<Pointer, std::vector<T1, T2>>,
              typename std::vector<T1, T2>::const_iterator>::value>::type>
typename std::vector<T1, T2>::const_iterator
f94(Iterator<Pointer, std::vector<T1, T2>> first,
    Iterator<Pointer, std::vector<T1, T2>> last) {
  return std::max_element(first, last);
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::const_iterator f95(const std::vector<T1, T2> &o) {
  return o.begin();
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::const_iterator f96(const std::vector<T1, T2> &o) {
  return o.end();
}

template <typename T1, typename T2 = std::allocator<T1>>
void f97(std::vector<T1, T2> &o, std::vector<T1, T2> &a0) {
  return o.swap(a0);
}

template <typename T1, typename T2 = std::allocator<T1>>
const T1 &f98(const std::vector<T1, T2> &o, std::size_t idx) {
  return o.at(idx);
}

template <typename T1, typename T2 = std::allocator<T1>>
const T1 &f99(const std::vector<T1, T2> &o) {
  return o.back();
}

template <typename T1, typename T2 = std::allocator<T1>>
void f100(std::vector<std::vector<T1, T2>> &o,
          const std::vector<T1, T2> &value) {
  return o.push_back(value);
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::iterator
f101(std::vector<T1, T2> &o, typename std::vector<T1, T2>::const_iterator pos,
     const T1 *first, const T1 *last) {
  return o.insert(pos, first, last);
}

template <typename T1, typename T2 = std::allocator<T1>>
void f102(std::vector<T1, T2> &o, std::size_t n,
          const typename std::vector<T1, T2>::value_type &value) {
  return o.resize(n, value);
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> &f103(std::vector<T1, T2> &dst, std::vector<T1, T2> &&src) {
  return dst.operator=(std::move(src));
}

template <typename T1, typename T2 = std::allocator<T1>>
typename std::vector<T1, T2>::const_iterator
f104(const std::vector<T1, T2> &o) {
  return o.cend();
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> &f105(std::vector<T1, T2> &dst,
                          const std::vector<T1, T2> &src) {
  return dst.operator=(src);
}

template <typename T1, typename T2 = std::allocator<T1>>
void f106(std::vector<T1, T2> &o) {
  return o.shrink_to_fit();
}

template <typename T1> std::vector<T1> f107(std::vector<T1> &&o) {
  return std::vector<T1>(std::move(o));
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> f108(std::vector<T1, T2> &&o) {
  return std::vector<T1, T2>(std::move(o));
}

template <typename T1> std::vector<T1> f109(const std::vector<T1> &o) {
  return std::vector<T1>(o);
}

template <typename T1, typename T2 = std::allocator<T1>>
std::vector<T1, T2> f110(const std::vector<T1, T2> &o) {
  return std::vector<T1, T2>(o);
}

template <typename T1>
std::vector<std::vector<T1>> &f111(std::vector<std::vector<T1>> &dst,
                                   std::vector<std::vector<T1>> &&src) {
  return dst.operator=(std::move(src));
}

template <typename T1, typename... Args>
T1 &f112(std::vector<T1> &o, Init<T1, Args> &&...args) {
  return o.emplace_back(std::forward<Args>(args)...);
}

template <typename T1, typename... Args>
std::vector<T1> &f113(std::vector<std::vector<T1>> &o,
                      Init<std::vector<T1>, Args> &&...args) {
  return o.emplace_back(std::forward<Args>(args)...);
}

template <typename T1, typename T2 = std::allocator<T1>, typename... Args>
T1 &f114(std::vector<T1, T2> &o, Init<T1, Args> &&...args) {
  return o.emplace_back(std::forward<Args>(args)...);
}
