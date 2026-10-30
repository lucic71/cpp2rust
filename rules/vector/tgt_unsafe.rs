// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

unsafe fn f67(a0: Vec<bool>) -> Vec<bool> {
    a0
}

unsafe fn f94<T1>() -> Vec<T1> {
    Vec::new()
}

unsafe fn f98<T1: Clone>(a0: Vec<T1>) -> Vec<T1> {
    a0.clone()
}

unsafe fn f99<T1>(a0: &mut Vec<T1>) -> Vec<T1> {
    std::mem::take(&mut *a0)
}

unsafe fn f103<T1: Clone>(a0: &mut Vec<T1>, a1: Vec<T1>) {
    *a0 = a1.clone()
}

unsafe fn f104<T1: Clone>(a0: &mut Vec<T1>, a1: &mut Vec<T1>) {
    *a0 = std::mem::take(&mut *a1)
}

unsafe fn f108<T1>(a0: &mut Vec<T1>) -> UnsafeRandomAccessIterator<T1> {
    a0.begin()
}

unsafe fn f109<T1>(a0: Vec<T1>) -> UnsafeRandomAccessIterator<T1> {
    a0.begin()
}

unsafe fn f110<T1>(a0: &mut Vec<T1>) -> UnsafeRandomAccessIterator<T1> {
    a0.end()
}

unsafe fn f111<T1>(a0: Vec<T1>) -> UnsafeRandomAccessIterator<T1> {
    a0.end()
}

unsafe fn f117<T1>(a0: Vec<T1>) -> UnsafeRandomAccessIterator<T1> {
    a0.end()
}

unsafe fn f120<T1>(a0: Vec<T1>) -> usize {
    a0.len()
}

unsafe fn f122<T1: Default>(a0: &mut Vec<T1>, a1: usize) {
    let __a0 = a1 as usize;
    a0.resize_with(__a0, || <T1>::default())
}

unsafe fn f123<T1: Default + Clone>(a0: &mut Vec<T1>, a1: usize, a2: T1) {
    let __a0 = a1 as usize;
    a0.resize(__a0, a2)
}

unsafe fn f124<T1>(a0: &mut Vec<T1>) {
    a0.shrink_to_fit()
}

unsafe fn f125<T1>(a0: Vec<T1>) -> usize {
    a0.capacity()
}

unsafe fn f126<T1>(a0: Vec<T1>) -> bool {
    a0.is_empty()
}

unsafe fn f127<T1>(a0: &mut Vec<T1>, a1: usize) {
    if a1 as usize > a0.capacity() as usize {
        let len_0 = a0.len();
        a0.reserve_exact(a1 as usize - len_0 as usize);
    }
}

unsafe fn f130<T1>(a0: &mut Vec<T1>, a1: usize) -> *mut T1 {
    (&mut (a0)[a1 as usize] as *mut T1)
}

unsafe fn f131<T1: Copy>(a0: &Vec<T1>, a1: usize) -> *const T1 {
    let __idx = a1;
    if __idx as usize >= a0.len() {
        panic!("out of bounds access")
    } else {
        (a0).as_ptr().add(__idx as usize)
    }
}

unsafe fn f132<T1>(a0: &mut Vec<T1>) -> *mut T1 {
    ((a0).first_mut().unwrap())
}

unsafe fn f134<T1>(a0: &mut Vec<T1>) -> *mut T1 {
    ((a0).last_mut().unwrap())
}

unsafe fn f135<T1>(a0: &Vec<T1>) -> &T1 {
    ((a0).last().unwrap())
}

unsafe fn f136<T1>(a0: &mut Vec<T1>) -> *mut T1 {
    a0.as_mut_ptr()
}

unsafe fn f137<T1>(a0: &mut Vec<T1>) -> *const T1 {
    a0.as_ptr()
}

unsafe fn f138<T1: Clone>(a0: &mut Vec<T1>, a1: T1) {
    let a0_clone = a1.clone();
    a0.push(a0_clone)
}

unsafe fn f139<T1: Default>(a0: &mut Vec<T1>, a1: &mut T1) {
    let __a1 = std::mem::take(&mut *a1);
    a0.push(__a1)
}

unsafe fn f140<T1>(a0: &mut Vec<T1>) {
    a0.pop();
}

unsafe fn f141<T1>(a0: &mut Vec<T1>, a1: UnsafeRandomAccessIterator<T1>, a2: T1) -> UnsafeRandomAccessIterator<T1> {
    let __it = a1;
    let pos = __it.iter_distance(&UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr())) as usize;
    a0.insert(pos, a2);
    __it
}

unsafe fn f142<T1>(a0: &mut Vec<T1>, a1: UnsafeRandomAccessIterator<T1>, a2: T1) -> UnsafeRandomAccessIterator<T1> {
    let __it = a1;
    let pos = __it.iter_distance(&UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr())) as usize;
    a0.insert(pos, a2);
    __it
}

unsafe fn f143<T1>(a0: &mut Vec<T1>, a1: UnsafeRandomAccessIterator<T1>, a2: Vec<T1>) -> UnsafeRandomAccessIterator<T1> {
    let __off = a1.iter_distance(&UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr())) as usize;
    a0.splice(__off..__off, a2);
    UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr().add(__off))
}

unsafe fn f144<T1: Clone>(a0: &mut Vec<T1>, a1: UnsafeRandomAccessIterator<T1>, a2: usize, a3: T1) -> UnsafeRandomAccessIterator<T1> {
    let __off = a1.iter_distance(&UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr())) as usize;
    a0.splice(__off..__off, std::iter::repeat_n(a3, a2));
    UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr().add(__off))
}

unsafe fn f145<T1>(a0: &mut Vec<T1>, a1: UnsafeRandomAccessIterator<T1>) -> UnsafeRandomAccessIterator<T1> {
    let __it = a1;
    let pos = __it.iter_distance(&UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr())) as usize;
    a0.remove(pos);
    __it
}

unsafe fn f147(a0: &mut Vec<bool>, a1: &mut Vec<bool>) {
    std::mem::swap(&mut *a0, &mut *a1)
}

unsafe fn f148<T1>(a0: &mut Vec<T1>) {
    a0.clear()
}

unsafe fn f153<T1>(a0: &mut Vec<T1>, init: T1) {
    let __init = init;
    a0.push(__init)
}

fn t4() -> usize {
    0_usize
}

fn t8() -> *mut bool {
    ::std::ptr::null_mut()
}

fn t22<T1>() -> Vec<T1> {
    Default::default()
}

fn t23<T1>() -> T1 {
    unimplemented!()
}

fn t2() -> Vec<bool> {
    Vec::new()
}

fn t6() -> bool {
    false
}

fn t24<T1>() -> *mut T1 {
    ::std::ptr::null_mut()
}

fn t25<T1>() -> *const T1 {
    ::std::ptr::null()
}

fn t26<T1>() -> *mut T1 {
    ::std::ptr::null_mut()
}

fn t27<T1>() -> *const T1 {
    ::std::ptr::null()
}

fn t28<T1>() -> UnsafeRandomAccessIterator<T1> {
    UnsafeRandomAccessIterator::default()
}

fn t29<T1>() -> UnsafeRandomAccessIterator<T1> {
    UnsafeRandomAccessIterator::default()
}

fn t32() -> usize {
    0_usize
}

fn t33() -> isize {
    0_isize
}

unsafe fn f128<T1, T2>(mut a0: &mut Vec<T1>, a1: usize) -> *mut T1 {
    let __idx = a1 as usize;
    let __v: &mut Vec<T1> = &mut a0;
    __v.as_mut_ptr().add(__idx)
}

unsafe fn f129<T1, T2>(a0: &Vec<T1>, a1: usize) -> *const T1 {
    let __idx = a1 as usize;
    let __v: &Vec<T1> = &a0;
    __v.as_ptr().add(__idx)
}

unsafe fn f96<T1: Default, T2>(a0: usize, a1: T2) -> Vec<T1> {
    (0..(a0) as usize)
        .map(|_| <T1>::default())
        .collect::<Vec<_>>()
}

unsafe fn f97<T1: Default + Clone, T2>(a0: usize, a1: T1, a2: T2) -> Vec<T1> {
    vec![a1; a0 as usize]
}

unsafe fn f102<T1, T2>(a0: Vec<T1>, a1: T2) -> Vec<T1> {
    a0
}

unsafe fn f149<T1: Clone, T2, T3: UnsafeCppInputIterator<T1>>(a0: T3, a1: T3, a2: T2) -> Vec<T1> {
    let mut __v = Vec::new();
    let mut __it = a0;
    while __it != a1 {
        __v.push((*__it.iter_deref()).clone());
        __it.iter_inc();
    }
    __v
}

unsafe fn f155<T1: Clone, T2, T3: UnsafeCppInputIterator<T1>>(
    a0: &mut Vec<T1>,
    a1: UnsafeRandomAccessIterator<T1>,
    a2: T3,
    a3: T3,
) -> UnsafeRandomAccessIterator<T1> {
    let __off = a1.iter_distance(&UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr())) as usize;
    let mut __tmp = Vec::new();
    let mut __it = a2;
    while __it != a3 {
        __tmp.push((*__it.iter_deref()).clone());
        __it.iter_inc();
    }
    a0.splice(__off..__off, __tmp);
    UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr().add(__off))
}

unsafe fn f12<T1>(a0: Vec<bool>, a1: T1) -> Vec<bool> {
    a0
}

unsafe fn f34<T1>(a0: &mut Vec<bool>, a1: usize) -> bool {
    let __idx = a1 as usize;
    let __v: &Vec<bool> = &a0;
    __v[__idx]
}

unsafe fn f66<T1>(a0: bool) -> bool {
    a0
}
