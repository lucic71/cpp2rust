// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <fenv.h>

typedef fenv_t t1;
typedef fexcept_t t2;

int f3(int a0) { return fesetround(a0); }
