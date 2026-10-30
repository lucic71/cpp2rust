// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

fn f1<
    T1: CppInputIterator<T3>,
    T2: CppIterator<T4>,
    T3: ByteRepr,
    T4: CppFrom<T3> + ByteRepr,
>(a0: T1, a1: T1, a2: T2) -> T2 {
    let mut __first = a0;
    let __last = a1;
    let mut __out = a2;
    while __first != __last {
        __out.iter_deref().write(T4::cpp_from(__first.iter_deref()));
        __first.iter_inc();
        __out.iter_inc();
    }
    __out
}

fn f16<T1: CppLess>(a0: Ptr<T1>, a1: Ptr<T1>) -> Ptr<T1> {
    let __a = a0;
    let __b = a1;
    if CppLess::less(&__a, &__b) { __b } else { __a }
}

fn f18<T1: CppLess>(a0: Ptr<T1>, a1: Ptr<T1>) -> Ptr<T1> {
    let __a = a0;
    let __b = a1;
    if CppLess::less(&__b, &__a) { __b } else { __a }
}
