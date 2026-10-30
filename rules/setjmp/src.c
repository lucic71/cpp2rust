// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <setjmp.h>

typedef jmp_buf t1;

int f1(jmp_buf env) { return _setjmp(env); }

void f2(jmp_buf env, int val) { return longjmp(env, val); }
