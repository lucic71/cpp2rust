// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::io::{Seek, Write};

unsafe fn f16<T1: PartialOrd>(a0: *const T1, a1: *const T1) -> *const T1 {
    let __a: *const _ = a0;
    let __b: *const _ = a1;
    if *__a < *__b { __b } else { __a }
}

unsafe fn f18<T1: PartialOrd>(a0: *const T1, a1: *const T1) -> *const T1 {
    let __a: *const _ = a0;
    let __b: *const _ = a1;
    if *__b < *__a { __b } else { __a }
}

unsafe fn f1<T1: UnsafeCppInputIterator<T3>, T2: UnsafeCppIterator<T4>, T3, T4: CppFromUnsafe<T3>>(
    a0: T1,
    a1: T1,
    a2: T2,
) -> T2 {
    let mut __first = a0;
    let __last = a1;
    let mut __out = a2;
    while __first != __last {
        *__out.iter_deref() = T4::cpp_from(__first.iter_deref());
        __first.iter_inc();
        __out.iter_inc();
    }
    __out
}
