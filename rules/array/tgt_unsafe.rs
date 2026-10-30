// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

unsafe fn f15<T1>(a0: Vec<T1>) -> usize {
    a0.len()
}

unsafe fn f26<T1>(a0: &mut Vec<T1>) -> *mut T1 {
    a0.as_mut_ptr()
}

unsafe fn f27<T1>(a0: Vec<T1>) -> *const T1 {
    a0.as_ptr()
}

fn t1<T1: Default + Clone, const T2: usize>() -> Vec<T1> {
    std::array::from_fn::<_, T2, _>(|_| <T1>::default()).to_vec()
}

fn t2<T1>() -> T1 {
    unimplemented!()
}

fn t3<T1>() -> *mut T1 {
    ::std::ptr::null_mut()
}

fn t4<T1>() -> *const T1 {
    ::std::ptr::null()
}

fn t7<T1>() -> UnsafeRandomAccessIterator<T1> {
    UnsafeRandomAccessIterator::default()
}

fn t8<T1>() -> UnsafeRandomAccessIterator<T1> {
    UnsafeRandomAccessIterator::default()
}

fn t9() -> usize {
    0_usize
}

unsafe fn f18<T1, const T2: usize>(mut a0: &mut Vec<T1>, a1: usize) -> *mut T1 {
    let __idx = a1 as usize;
    let __v: &mut Vec<T1> = &mut a0;
    __v.as_mut_ptr().add(__idx)
}
