// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn t1<T1>() -> Deque<T1> {
    Default::default()
}

fn t4<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn t5<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn f1<T1: 'static>(a0: Ptr<Deque<T1>>) -> Ptr<T1> {
    a0.back()
}

fn f2<T1: 'static>(a0: Ptr<Deque<T1>>) -> Ptr<T1> {
    a0.front()
}

fn f3<T1>(a0: Deque<T1>) -> bool {
    a0.is_empty()
}

fn f4<T1: Default>(a0: &mut Deque<T1>, a1: &mut T1) {
    a0.push_back(std::mem::take(&mut *a1))
}

fn f5<T1>(a0: &mut Deque<T1>) {
    a0.pop_front()
}

fn f7<T1>(a0: &mut Deque<T1>, a1: T1) {
    a0.push_back(a1)
}

fn f8<T1: Clone>(a0: Deque<T1>) -> Deque<T1> {
    a0.clone()
}

fn f9<T1>(a0: &mut Deque<T1>) -> Deque<T1> {
    std::mem::take(&mut *a0)
}

fn f10<T1: Clone + 'static>(a0: Ptr<Deque<T1>>, a1: Deque<T1>) {
    a0.write(a1)
}

fn f11<T1: 'static>(a0: Ptr<Deque<T1>>, a1: &mut Deque<T1>) {
    a0.write(std::mem::take(&mut *a1))
}

fn f12<T1>(a0: &mut Deque<T1>, init: T1) {
    let __init = init;
    a0.push_back(__init)
}

fn f14<T1>() -> Deque<T1> {
    Deque::new()
}

fn f15<T1>(a0: &mut Deque<T1>, a1: T1) {
    a0.push_front(a1)
}

fn f16<T1: Default>(a0: &mut Deque<T1>, a1: &mut T1) {
    a0.push_front(std::mem::take(&mut *a1))
}

fn f17<T1>(a0: &mut Deque<T1>, init: T1) {
    let __init = init;
    a0.push_front(__init)
}

fn f18<T1>(a0: &mut Deque<T1>) {
    a0.pop_back()
}

fn f19<T1>(a0: Deque<T1>) -> usize {
    a0.len()
}

fn f20<T1: 'static>(a0: Ptr<Deque<T1>>, a1: usize) -> Ptr<T1> {
    a0.get(a1)
}

fn f21<T1: 'static>(a0: Ptr<Deque<T1>>, a1: usize) -> Ptr<T1> {
    a0.get(a1)
}

fn f22<T1: 'static>(a0: Ptr<Deque<T1>>, a1: usize) -> Ptr<T1> {
    a0.at(a1)
}

fn f23<T1: 'static>(a0: Ptr<Deque<T1>>, a1: usize) -> Ptr<T1> {
    a0.at(a1)
}

fn f24<T1: 'static>(a0: Ptr<Deque<T1>>) -> Ptr<T1> {
    a0.front()
}

fn f25<T1: 'static>(a0: Ptr<Deque<T1>>) -> Ptr<T1> {
    a0.back()
}

fn f26<T1: 'static>(a0: Ptr<Deque<T1>>) -> RandomAccessIterator<T1> {
    a0.begin()
}

fn f27<T1: 'static>(a0: Ptr<Deque<T1>>) -> RandomAccessIterator<T1> {
    a0.end()
}

fn f28<T1: 'static>(a0: Ptr<Deque<T1>>) -> RandomAccessIterator<T1> {
    a0.begin()
}

fn f29<T1: 'static>(a0: Ptr<Deque<T1>>) -> RandomAccessIterator<T1> {
    a0.end()
}

fn f30<T1>(a0: &mut Deque<T1>) {
    a0.clear()
}

fn f31<T1>(a0: &mut Deque<T1>, a1: RandomAccessIterator<T1>, a2: T1) -> RandomAccessIterator<T1> {
    let __off = a1.index();
    a0.insert(__off, a2);
    a1
}

fn f32<T1: Default>(
    a0: &mut Deque<T1>,
    a1: RandomAccessIterator<T1>,
    a2: &mut T1,
) -> RandomAccessIterator<T1> {
    let __off = a1.index();
    a0.insert(__off, std::mem::take(&mut *a2));
    a1
}

fn f33<T1>(a0: &mut Deque<T1>, a1: RandomAccessIterator<T1>) -> RandomAccessIterator<T1> {
    let __off = a1.index();
    a0.erase(__off);
    a1
}

fn f34<T1: Default>(a0: usize) -> Deque<T1> {
    Deque::from_fn(a0, <T1>::default)
}

fn f35<T1: Clone>(a0: usize, a1: T1) -> Deque<T1> {
    let __value = a1;
    Deque::from_fn(a0, || __value.clone())
}
