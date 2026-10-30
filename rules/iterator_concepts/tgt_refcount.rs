// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1<T1: CppBidirectionalIterator<T2>, T2>(a0: &mut T1) -> T1 {
    a0.iter_dec()
}

fn f2<T1: CppBidirectionalIterator<T2>, T2>(a0: &mut T1) -> T1 {
    a0.iter_post_dec()
}

fn f3<T1: CppInputIterator<T2>, T2>(a0: T1) -> Ptr<T2> {
    a0.iter_deref()
}

fn f11<T1: CppIterator<T2>, T2>(a0: &mut T1, a1: T1) {
    *a0 = a1.clone()
}

fn f12<T1: CppIterator<T2>, T2>(a0: &mut T1, a1: T1) {
    *a0 = a1.clone()
}

fn f13<T1: CppIterator<T2>, T2>(a0: &mut T1, a1: T1) {
    *a0 = a1.clone()
}

fn f14<T1: CppInputIterator<T2>, T2>(a0: T1, a1: T1) -> bool {
    a0 == a1
}

fn f15<T1: CppInputIterator<T2>, T2>(a0: T1, a1: T1) -> bool {
    a0 != a1
}

fn f16<T1: CppIterator<T2>, T2>(a0: &mut T1) -> T1 {
    a0.iter_post_inc()
}

fn f17<T1: CppIterator<T2>, T2>(a0: T1) -> Ptr<T2> {
    a0.iter_deref()
}

fn f18<T1: CppIterator<T2>, T2>(a0: &mut T1, a1: T1) {
    *a0 = a1
}

fn f20<T1: CppIterator<T2>, T2>(a0: &mut T1) -> T1 {
    a0.iter_inc()
}

fn f21<T1: CppIterator<T2>, T2>(a0: &mut T1) -> T1 {
    a0.iter_post_inc()
}

fn f27<T1: CppRandomAccessIterator<T2>, T2>(a0: T1, a1: T1) -> bool {
    a0 < a1
}

fn f28<T1: CppRandomAccessIterator<T2>, T2>(a0: T1, a1: T1) -> bool {
    a0 > a1
}

fn f29<T1: CppRandomAccessIterator<T2>, T2>(a0: T1, a1: T1) -> bool {
    a0 <= a1
}

fn f30<T1: CppRandomAccessIterator<T2>, T2>(a0: T1, a1: T1) -> bool {
    a0 >= a1
}

fn f31<T1: CppRandomAccessIterator<T2>, T2>(a0: &mut T1, a1: isize) -> T1 {
    a0.iter_add_assign(a1)
}

fn f32<T1: CppRandomAccessIterator<T2>, T2>(a0: T1, a1: isize) -> T1 {
    a0.iter_add(a1)
}

fn f33<T1: CppRandomAccessIterator<T2>, T2>(a0: isize, a1: T1) -> T1 {
    a1.iter_add(a0)
}

fn f34<T1: CppRandomAccessIterator<T2>, T2>(a0: &mut T1, a1: isize) -> T1 {
    a0.iter_add_assign(-a1)
}

fn f35<T1: CppRandomAccessIterator<T2>, T2>(a0: T1, a1: isize) -> T1 {
    a0.iter_add(-a1)
}

fn f36<T1: CppRandomAccessIterator<T2>, T2>(a0: T1, a1: isize) -> Ptr<T2> {
    a0.iter_at(a1)
}

fn f38<T1: CppInputIterator<T3> + PartialEq<T2>, T2: CppInputIterator<T3>, T3>(a0: T1, a1: T2) -> bool {
    a0 == a1
}

fn f39<T1: CppInputIterator<T3> + PartialEq<T2>, T2: CppInputIterator<T3>, T3>(a0: T1, a1: T2) -> bool {
    a0 != a1
}

fn f40<T1: CppRandomAccessIterator<T3>, T2: Into<T1>, T3>(a0: T1, a1: T2) -> isize {
    a0.iter_distance(&a1.into())
}

fn t1<T1: CppIterator<T2>, T2>() -> BidirectionalIterator<T2> {
    BidirectionalIterator::default()
}

fn t2<T1: CppIterator<T2>, T2>() -> RandomAccessIterator<T2> {
    RandomAccessIterator::default()
}
