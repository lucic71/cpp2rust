// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

unsafe fn f10<T2, T3>(a0: &mut (T2, T3)) -> *mut T2 {
    &mut a0.0
}

unsafe fn f11<T2, T3>(a0: &(T2, T3)) -> *const T2 {
    &a0.0
}

unsafe fn f12<T2, T3>(a0: &mut (T2, T3)) -> *mut T2 {
    &mut a0.0
}

unsafe fn f13<T1, T2>(a0: T1, a1: T2) -> (T1, T2) {
    (a0, a1)
}
