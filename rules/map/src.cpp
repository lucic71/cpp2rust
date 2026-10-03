// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <iterator>
#include <map>
#include <type_traits>
#include <utility>

template <typename T1, typename T2> using t1 = std::map<T1, T2>;

template <
    typename It,
    typename T1 = typename std::remove_const<
        typename std::iterator_traits<It>::value_type::first_type>::type,
    typename T2 = typename std::iterator_traits<It>::value_type::second_type,
    typename = typename std::enable_if<std::is_same<
        It, typename std::map<T1, T2>::const_iterator>::value>::type>
void t2(It *);

template <
    typename It,
    typename T1 = typename std::remove_const<
        typename std::iterator_traits<It>::value_type::first_type>::type,
    typename T2 = typename std::iterator_traits<It>::value_type::second_type,
    typename = typename std::enable_if<
        std::is_same<It, typename std::map<T1, T2>::iterator>::value>::type>
void t3(It *);

template <typename T1, typename T2> T2 &f1(std::map<T1, T2> &o, const T1 &key) {
  return o.operator[](key);
}

template <typename T1, typename T2> std::size_t f2(const std::map<T1, T2> &o) {
  return o.size();
}

template <typename T1, typename T2>
typename std::map<T1, T2>::iterator f3(std::map<T1, T2> &o,
                                       typename std::map<T1, T2>::iterator it) {
  return o.erase(it);
}

template <typename T1, typename T2> std::map<T1, T2> f5() {
  return std::map<T1, T2>();
}

template <typename T1, typename T2>
std::map<T1, T2> f6(const std::map<T1, T2> &&o) {
  return std::map<T1, T2>(std::move(o));
}

template <typename T1, typename T2> T2 &f7(std::map<T1, T2> &o, const T1 &key) {
  return o.at(key);
}

template <typename T1, typename T2> T2 &f8(std::map<T1, T2> &o, T1 &&key) {
  return o.operator[](std::move(key));
}

template <typename T1, typename T2>
typename std::map<T1, T2>::const_iterator f9(const std::map<T1, T2> &o) {
  return o.end();
}

template <typename T1, typename T2>
typename std::map<T1, T2>::iterator f10(std::map<T1, T2> &o, const T1 &key) {
  return o.find(key);
}

template <
    typename It,
    typename T1 = typename std::remove_const<
        typename std::iterator_traits<It>::value_type::first_type>::type,
    typename T2 = typename std::iterator_traits<It>::value_type::second_type,
    typename = typename std::enable_if<
        std::is_same<It, typename std::map<T1, T2>::iterator>::value>::type>
bool f11(It a, It b) {
  return operator!=(a, b);
}

template <typename T1, typename T2>
typename std::map<T1, T2>::iterator f12(std::map<T1, T2> &o) {
  return o.begin();
}

template <
    typename It,
    typename T1 = typename std::remove_const<
        typename std::iterator_traits<It>::value_type::first_type>::type,
    typename T2 = typename std::iterator_traits<It>::value_type::second_type,
    typename = typename std::enable_if<std::is_same<
        It, typename std::map<T1, T2>::const_iterator>::value>::type>
bool f13(It a, It b) {
  return operator==(a, b);
}

template <typename T1, typename T2>
typename std::map<T1, T2>::iterator f14(std::map<T1, T2> &o) {
  return o.end();
}

template <typename T1, typename T2>
const T2 &f15(const std::map<T1, T2> &o, const T1 &key) {
  return o.at(key);
}

template <
    typename It,
    typename T1 = typename std::remove_const<
        typename std::iterator_traits<It>::value_type::first_type>::type,
    typename T2 = typename std::iterator_traits<It>::value_type::second_type,
    typename = typename std::enable_if<
        std::is_same<It, typename std::map<T1, T2>::iterator>::value>::type>
bool f16(It a, It b) {
  return operator==(a, b);
}

template <typename T1, typename T2>
typename std::map<T1, T2>::const_iterator f17(const std::map<T1, T2> &o,
                                              const T1 &key) {
  return o.find(key);
}

template <
    typename It,
    typename T1 = typename std::remove_const<
        typename std::iterator_traits<It>::value_type::first_type>::type,
    typename T2 = typename std::iterator_traits<It>::value_type::second_type,
    typename = typename std::enable_if<
        std::is_same<It, typename std::map<T1, T2>::iterator>::value>::type>
typename std::map<T1, T2>::const_iterator f19(const It &it) {
  return typename std::map<T1, T2>::const_iterator(it);
}

template <
    typename It,
    typename T1 = typename std::remove_const<
        typename std::iterator_traits<It>::value_type::first_type>::type,
    typename T2 = typename std::iterator_traits<It>::value_type::second_type,
    typename = typename std::enable_if<std::is_same<
        It, typename std::map<T1, T2>::const_iterator>::value>::type>
const T1 &f20(It it) {
  return it->first;
}

template <
    typename It,
    typename T1 = typename std::remove_const<
        typename std::iterator_traits<It>::value_type::first_type>::type,
    typename T2 = typename std::iterator_traits<It>::value_type::second_type,
    typename = typename std::enable_if<std::is_same<
        It, typename std::map<T1, T2>::const_iterator>::value>::type>
const T2 &f21(It it) {
  return it->second;
}

template <
    typename It,
    typename T1 = typename std::remove_const<
        typename std::iterator_traits<It>::value_type::first_type>::type,
    typename T2 = typename std::iterator_traits<It>::value_type::second_type,
    typename = typename std::enable_if<
        std::is_same<It, typename std::map<T1, T2>::iterator>::value>::type>
const T1 &f22(It it) {
  return it->first;
}

template <
    typename It,
    typename T1 = typename std::remove_const<
        typename std::iterator_traits<It>::value_type::first_type>::type,
    typename T2 = typename std::iterator_traits<It>::value_type::second_type,
    typename = typename std::enable_if<
        std::is_same<It, typename std::map<T1, T2>::iterator>::value>::type>
T2 &f23(It it) {
  return it->second;
}

template <typename T1, typename T2> std::map<T1, T2> f24(std::map<T1, T2> &&o) {
  return std::map<T1, T2>(std::move(o));
}

template <typename T1, typename T2>
std::map<T1, T2> &f25(std::map<T1, T2> &dst, std::map<T1, T2> &&src) {
  return dst.operator=(std::move(src));
}

template <typename T1, typename T2>
std::map<T1, T2> &f26(std::map<T1, T2> &dst, const std::map<T1, T2> &src) {
  return dst.operator=(src);
}
