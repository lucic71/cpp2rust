// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

unsafe fn f1<T1: UnsafeCppBidirectionalIterator<T2>, T2>(a0: &mut T1) -> T1 {
    a0.iter_dec()
}

unsafe fn f2<T1: UnsafeCppBidirectionalIterator<T2>, T2>(a0: &mut T1) -> T1 {
    a0.iter_post_dec()
}

unsafe fn f3<T1: UnsafeCppInputIterator<T2>, T2>(a0: T1) -> *mut T2 {
    a0.iter_deref()
}

unsafe fn f11<T1: UnsafeCppIterator<T2>, T2>(a0: &mut T1, a1: T1) {
    *a0 = a1.clone()
}

unsafe fn f12<T1: UnsafeCppIterator<T2>, T2>(a0: &mut T1, a1: T1) {
    *a0 = a1.clone()
}

unsafe fn f13<T1: UnsafeCppIterator<T2>, T2>(a0: &mut T1, a1: T1) {
    *a0 = a1.clone()
}

unsafe fn f14<T1: UnsafeCppInputIterator<T2>, T2>(a0: T1, a1: T1) -> bool {
    a0 == a1
}

unsafe fn f15<T1: UnsafeCppInputIterator<T2>, T2>(a0: T1, a1: T1) -> bool {
    a0 != a1
}

unsafe fn f16<T1: UnsafeCppIterator<T2>, T2>(a0: &mut T1) -> T1 {
    a0.iter_post_inc()
}

unsafe fn f17<T1: UnsafeCppIterator<T2>, T2>(a0: T1) -> *mut T2 {
    a0.iter_deref()
}

unsafe fn f18<T1: UnsafeCppIterator<T2>, T2>(a0: &mut T1, a1: T1) {
    *a0 = a1
}

unsafe fn f20<T1: UnsafeCppIterator<T2>, T2>(a0: &mut T1) -> T1 {
    a0.iter_inc()
}

unsafe fn f21<T1: UnsafeCppIterator<T2>, T2>(a0: &mut T1) -> T1 {
    a0.iter_post_inc()
}

unsafe fn f27<T1: UnsafeCppRandomAccessIterator<T2>, T2>(a0: T1, a1: T1) -> bool {
    a0 < a1
}

unsafe fn f28<T1: UnsafeCppRandomAccessIterator<T2>, T2>(a0: T1, a1: T1) -> bool {
    a0 > a1
}

unsafe fn f29<T1: UnsafeCppRandomAccessIterator<T2>, T2>(a0: T1, a1: T1) -> bool {
    a0 <= a1
}

unsafe fn f30<T1: UnsafeCppRandomAccessIterator<T2>, T2>(a0: T1, a1: T1) -> bool {
    a0 >= a1
}

unsafe fn f31<T1: UnsafeCppRandomAccessIterator<T2>, T2>(a0: &mut T1, a1: isize) -> T1 {
    a0.iter_add_assign(a1)
}

unsafe fn f32<T1: UnsafeCppRandomAccessIterator<T2>, T2>(a0: T1, a1: isize) -> T1 {
    a0.iter_add(a1)
}

unsafe fn f33<T1: UnsafeCppRandomAccessIterator<T2>, T2>(a0: isize, a1: T1) -> T1 {
    a1.iter_add(a0)
}

unsafe fn f34<T1: UnsafeCppRandomAccessIterator<T2>, T2>(a0: &mut T1, a1: isize) -> T1 {
    a0.iter_add_assign(-a1)
}

unsafe fn f35<T1: UnsafeCppRandomAccessIterator<T2>, T2>(a0: T1, a1: isize) -> T1 {
    a0.iter_add(-a1)
}

unsafe fn f36<T1: UnsafeCppRandomAccessIterator<T2>, T2>(a0: T1, a1: isize) -> *mut T2 {
    a0.iter_at(a1)
}

unsafe fn f38<T1: UnsafeCppInputIterator<T3> + PartialEq<T2>, T2: UnsafeCppInputIterator<T3>, T3>(a0: T1, a1: T2) -> bool {
    a0 == a1
}

unsafe fn f39<T1: UnsafeCppInputIterator<T3> + PartialEq<T2>, T2: UnsafeCppInputIterator<T3>, T3>(a0: T1, a1: T2) -> bool {
    a0 != a1
}

unsafe fn f40<T1: UnsafeCppRandomAccessIterator<T3>, T2: Into<T1>, T3>(a0: T1, a1: T2) -> isize {
    a0.iter_distance(&a1.into())
}

fn t1<T1: UnsafeCppIterator<T2>, T2>() -> UnsafeBidirectionalIterator<T2> {
    UnsafeBidirectionalIterator::default()
}

fn t2<T1: UnsafeCppIterator<T2>, T2>() -> UnsafeRandomAccessIterator<T2> {
    UnsafeRandomAccessIterator::default()
}
