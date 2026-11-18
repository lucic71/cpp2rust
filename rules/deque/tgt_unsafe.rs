// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1<T1>() -> VecDeque<T1> {
    Default::default()
}

fn t2() -> usize {
    0_usize
}

fn t3<T1>() -> T1 {
    unimplemented!()
}

fn t4<T1>() -> *mut T1 {
    ::std::ptr::null_mut()
}

fn t5<T1>() -> *const T1 {
    ::std::ptr::null()
}

unsafe fn f1<T1>(a0: &mut VecDeque<T1>) -> *const T1 {
    (a0.back_mut().unwrap())
}

unsafe fn f2<T1>(a0: &mut VecDeque<T1>) -> *const T1 {
    ((a0).front_mut().unwrap())
}

unsafe fn f3<T1>(a0: VecDeque<T1>) -> bool {
    a0.is_empty()
}

unsafe fn f4<T1: Default>(a0: &mut VecDeque<T1>, a1: &mut T1) {
    a0.push_back(std::mem::take(&mut *a1))
}

unsafe fn f5<T1>(a0: &mut VecDeque<T1>) {
    a0.pop_front();
}

unsafe fn f7<T1>(a0: &mut VecDeque<T1>, a1: T1) {
    a0.push_back(a1)
}

unsafe fn f8<T1: Clone>(a0: VecDeque<T1>) -> VecDeque<T1> {
    a0.clone()
}

unsafe fn f9<T1>(a0: &mut VecDeque<T1>) -> VecDeque<T1> {
    std::mem::take(&mut *a0)
}

unsafe fn f10<T1: Clone>(
    a0: &mut VecDeque<T1>,
    a1: VecDeque<T1>,
) {
    *a0 = a1.clone()
}

unsafe fn f11<T1>(
    a0: &mut VecDeque<T1>,
    a1: &mut VecDeque<T1>,
) {
    *a0 = std::mem::take(&mut *a1)
}

unsafe fn f12<T1>(a0: &mut VecDeque<T1>, init: T1) {
    let __init = init;
    a0.push_back(__init)
}

unsafe fn f14<T1>() -> VecDeque<T1> {
    VecDeque::new()
}

unsafe fn f15<T1>(a0: &mut VecDeque<T1>, a1: T1) {
    a0.push_front(a1)
}

unsafe fn f16<T1: Default>(a0: &mut VecDeque<T1>, a1: &mut T1) {
    a0.push_front(std::mem::take(&mut *a1))
}

unsafe fn f17<T1>(a0: &mut VecDeque<T1>, init: T1) {
    let __init = init;
    a0.push_front(__init)
}

unsafe fn f18<T1>(a0: &mut VecDeque<T1>) {
    a0.pop_back();
}

unsafe fn f19<T1>(a0: VecDeque<T1>) -> usize {
    a0.len()
}

unsafe fn f20<T1>(a0: &mut VecDeque<T1>, a1: usize) -> *mut T1 {
    (&mut (a0)[a1] as *mut T1)
}

unsafe fn f21<T1>(a0: &VecDeque<T1>, a1: usize) -> *const T1 {
    (&(a0)[a1] as *const T1)
}

unsafe fn f22<T1>(a0: &mut VecDeque<T1>, a1: usize) -> *mut T1 {
    (a0.get_mut(a1).expect("out of bounds access") as *mut T1)
}

unsafe fn f23<T1>(a0: &VecDeque<T1>, a1: usize) -> *const T1 {
    (a0.get(a1).expect("out of bounds access") as *const T1)
}

unsafe fn f24<T1>(a0: &VecDeque<T1>) -> *const T1 {
    (a0.front().unwrap())
}

unsafe fn f25<T1>(a0: &VecDeque<T1>) -> *const T1 {
    (a0.back().unwrap())
}

unsafe fn f26<T1>(a0: &mut VecDeque<T1>) -> UnsafeRandomAccessIterator<T1> {
    a0.begin()
}

unsafe fn f27<T1>(a0: &mut VecDeque<T1>) -> UnsafeRandomAccessIterator<T1> {
    a0.end()
}

unsafe fn f28<T1>(a0: VecDeque<T1>) -> UnsafeRandomAccessIterator<T1> {
    a0.begin()
}

unsafe fn f29<T1>(a0: VecDeque<T1>) -> UnsafeRandomAccessIterator<T1> {
    a0.end()
}

unsafe fn f30<T1>(a0: &mut VecDeque<T1>) {
    a0.clear()
}

unsafe fn f31<T1>(
    a0: &mut VecDeque<T1>,
    a1: UnsafeRandomAccessIterator<T1>,
    a2: T1,
) -> UnsafeRandomAccessIterator<T1> {
    let __d = &mut *a0;
    let __off = a1.iter_distance(&__d.begin()) as usize;
    __d.insert(__off, a2);
    a1
}

unsafe fn f32<T1: Default>(
    a0: &mut VecDeque<T1>,
    a1: UnsafeRandomAccessIterator<T1>,
    a2: &mut T1,
) -> UnsafeRandomAccessIterator<T1> {
    let __d = &mut *a0;
    let __off = a1.iter_distance(&__d.begin()) as usize;
    __d.insert(__off, std::mem::take(&mut *a2));
    a1
}

unsafe fn f33<T1>(
    a0: &mut VecDeque<T1>,
    a1: UnsafeRandomAccessIterator<T1>,
) -> UnsafeRandomAccessIterator<T1> {
    let __d = &mut *a0;
    let __off = a1.iter_distance(&__d.begin()) as usize;
    __d.remove(__off);
    a1
}

unsafe fn f34<T1: Default>(a0: usize) -> VecDeque<T1> {
    (0..a0)
        .map(|_| <T1>::default())
        .collect::<VecDeque<_>>()
}

unsafe fn f35<T1: Clone>(a0: usize, a1: T1) -> VecDeque<T1> {
    std::iter::repeat_n(a1, a0).collect::<VecDeque<_>>()
}
