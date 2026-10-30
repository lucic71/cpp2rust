// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn f26<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f27<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn t3<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn t4<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn t7<T1>() -> RandomAccessIterator<T1> {
    RandomAccessIterator::default()
}

fn t8<T1>() -> RandomAccessIterator<T1> {
    RandomAccessIterator::default()
}

fn f18<T1, const T2: usize>(a0: Ptr<T1>, a1: usize) -> Ptr<T1> {
    a0.offset(a1 as isize)
}
