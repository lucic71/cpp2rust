#include <iterator>
#include <type_traits>

template <typename T1>
  requires std::input_iterator<T1> && (std::is_class<T1>::value)
typename std::iterator_traits<T1>::pointer f1(const T1 &a0) {
  return a0.operator->();
}
