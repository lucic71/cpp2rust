// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

unsafe fn f3<T1, T2>() -> UnsafeMap<T1, (T1, T2)> {
    UnsafeMap::new()
}

unsafe fn f5<T1: Ord + Clone, T2: Clone>(a0: UnsafeMap<T1, (T1, T2)>) -> UnsafeMap<T1, (T1, T2)> {
    a0.clone()
}

unsafe fn f6<T1, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>) -> UnsafeMap<T1, (T1, T2)> {
    std::mem::take(&mut *a0)
}

unsafe fn f12<T1: Ord + Clone, T2: Clone>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: UnsafeMap<T1, (T1, T2)>) {
    *a0 = a1.clone()
}

unsafe fn f13<T1, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: &mut UnsafeMap<T1, (T1, T2)>) {
    *a0 = std::mem::take(&mut *a1)
}

unsafe fn f16<T1, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>) -> UnsafeBidirectionalIterator<(T1, T2)> {
    a0.begin()
}

unsafe fn f17<T1, T2>(a0: UnsafeMap<T1, (T1, T2)>) -> UnsafeBidirectionalIterator<(T1, T2)> {
    a0.begin()
}

unsafe fn f18<T1, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>) -> UnsafeBidirectionalIterator<(T1, T2)> {
    a0.end()
}

unsafe fn f19<T1, T2>(a0: UnsafeMap<T1, (T1, T2)>) -> UnsafeBidirectionalIterator<(T1, T2)> {
    a0.end()
}

unsafe fn f29<T1, T2>(a0: UnsafeMap<T1, (T1, T2)>) -> usize {
    a0.len()
}

unsafe fn f31<T1: Ord + Clone, T2: Default>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: T1) -> &mut T2 {
    &mut (*a0.subscript(&a1))
}

unsafe fn f32<T1: Ord, T2: Default>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: *mut T1) -> &mut T2 {
    &mut (*a0.subscript_move(a1, |__k| cpp_move_unsafe!(__k)))
}

unsafe fn f33<T1: Ord, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: T1) -> *const T2 {
    &raw const (*a0.get(&a1).expect("out of range!")).1
}

unsafe fn f34<T1: Ord, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: T1) -> *const T2 {
    &raw const (*a0.get(&a1).expect("out of range!")).1
}

unsafe fn f44<T1: Ord, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: UnsafeBidirectionalIterator<(T1, T2)>) -> UnsafeBidirectionalIterator<(T1, T2)> {
    a0.erase(&a1)
}

unsafe fn f45<T1: Ord, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: UnsafeBidirectionalIterator<(T1, T2)>) -> UnsafeBidirectionalIterator<(T1, T2)> {
    a0.erase(&a1)
}

unsafe fn f46<T1: Ord, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: T1) -> usize {
    a0.erase_key(&a1)
}

unsafe fn f52<T1: Ord, T2>(a0: &mut UnsafeMap<T1, (T1, T2)>, a1: T1) -> UnsafeBidirectionalIterator<(T1, T2)> {
    a0.find(&a1)
}

unsafe fn f53<T1: Ord, T2>(a0: UnsafeMap<T1, (T1, T2)>, a1: T1) -> UnsafeBidirectionalIterator<(T1, T2)> {
    a0.find(&a1)
}

fn t1<T1, T2>() -> UnsafeMap<T1, (T1, T2)> {
    UnsafeMap::new()
}

fn t2<T1>() -> T1 {
    unimplemented!()
}

fn t3<T1, T2>() -> T2 {
    unimplemented!()
}

fn t14() -> usize {
    0_usize
}

fn t12<T1, T2>() -> UnsafeBidirectionalIterator<(T1, T2)> {
    UnsafeBidirectionalIterator::default()
}

fn t13<T1, T2>() -> UnsafeBidirectionalIterator<(T1, T2)> {
    UnsafeBidirectionalIterator::default()
}
