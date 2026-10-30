// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

unsafe fn f1<T1: Clone, T2: Clone>(a0: (T1, T2)) -> (T1, T2) {
    a0.clone()
}

unsafe fn f2<T1: Default, T2: Default>(a0: &mut (T1, T2)) -> (T1, T2) {
    std::mem::take(&mut *a0)
}

unsafe fn f7<T1: Clone, T2: Clone>(a0: &mut (T1, T2), a1: (T1, T2)) {
    *a0 = a1.clone()
}

unsafe fn f8<T1: Default, T2: Default>(a0: &mut (T1, T2), a1: &mut (T1, T2)) {
    *a0 = std::mem::take(&mut *a1)
}

unsafe fn f13<T1: CppFromUnsafe<T3>, T2: CppFromUnsafe<T4>, T3, T4>(a0: &(T3, T4)) -> (T1, T2) {
    let __a0 = a0;
    (T1::cpp_from(&__a0.0), T2::cpp_from(&__a0.1))
}

unsafe fn f14<T1: CppFromUnsafe<T3>, T2: CppFromUnsafe<T4>, T3, T4>(a0: &mut (T3, T4)) -> (T1, T2) {
    let mut __a0 = a0;
    (T1::cpp_from_move(&mut __a0.0), T2::cpp_from_move(&mut __a0.1))
}

unsafe fn f18<T1: CppFromUnsafe<T3>, T2: CppFromUnsafe<T4>, T3, T4>(a0: &mut (T1, T2), a1: &(T3, T4)) {
    let __a1 = a1;
    *a0 = (T1::cpp_from(&__a1.0), T2::cpp_from(&__a1.1))
}

unsafe fn f19<T1: CppFromUnsafe<T3>, T2: CppFromUnsafe<T4>, T3, T4>(a0: &mut (T1, T2), a1: &mut (T3, T4)) {
    let mut __a1 = a1;
    *a0 = (T1::cpp_from_move(&mut __a1.0), T2::cpp_from_move(&mut __a1.1))
}

fn t1<T1: Default, T2: Default>() -> (T1, T2) {
    <(T1, T2)>::default()
}

unsafe fn f12<T1, T2, T3, T4>(a0: T1, a1: T2) -> (T1, T2) {
    (a0, a1)
}

unsafe fn f15<T1: CppFromUnsafe<T3>, T2: CppFromUnsafe<T4>, T3, T4>(a0: &mut (T3, T4)) -> (T1, T2) {
    let __a0 = a0;
    (T1::cpp_from(&__a0.0), T2::cpp_from(&__a0.1))
}
