#pragma once

#define CAT_(a, b) a##b
#define CAT(a, b) CAT_(a, b)
#define FN(name) CAT(name, SUFFIX)

#define SUFFIX A
#define FACTOR 2
#include "template_inc.h"
#undef SUFFIX
#undef FACTOR

#define SUFFIX B
#define FACTOR 3
#include "template_inc.h"
#undef SUFFIX
#undef FACTOR

#define DEFINE_PAIR(T)                                        \
  static T CAT(pmin_, T)(T a, T b) { return a < b ? a : b; }  \
  static T CAT(pmax_, T)(T a, T b) { return a > b ? a : b; }

DEFINE_PAIR(int)
DEFINE_PAIR(long)
