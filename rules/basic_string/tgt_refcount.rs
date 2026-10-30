// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn f10() -> Vec<i8> {
    vec![0]
}

fn f12(a0: Vec<i8>) -> Vec<i8> {
    a0
}

fn f17(a0: &mut Vec<i8>) -> Vec<i8> {
    std::mem::take(&mut *a0)
}

fn f21(a0: Ptr<Vec<i8>>, a1: Vec<i8>) {
    a0.write(a1)
}

fn f24(a0: Ptr<Vec<i8>>, a1: &mut Vec<i8>) {
    a0.write(std::mem::take(&mut *a1))
}

fn f27(a0: Ptr<i8>) -> RandomAccessIterator<i8> {
    RandomAccessIterator::Vec(a0)
}

fn f29(a0: Ptr<i8>) -> RandomAccessIterator<i8> {
    RandomAccessIterator::Vec(a0.to_last())
}

fn f39(a0: Vec<i8>) -> usize {
    (a0.len() - 1)
}

fn f40(a0: Vec<i8>) -> usize {
    (a0.len() - 1)
}

fn f43(a0: &mut Vec<i8>, a1: usize) {
    a0.pop();
    a0.resize((a1) as usize, 0);
    a0.push(0)
}

fn f44(a0: &mut Vec<i8>) {
    a0.shrink_to_fit()
}

fn f48(a0: &mut Vec<i8>) {
    a0.clear();
    a0.push(0)
}

fn f49(a0: Vec<i8>) -> bool {
    a0.len() <= 1
}

fn f53(a0: Ptr<Vec<i8>>, a1: usize) -> Ptr<i8> {
    if a1 as usize >= (*a0.upgrade().deref()).len().saturating_sub(1) {
        panic!("out of bounds access")
    } else {
        a0.decay().offset(a1 as isize)
    }
}

fn f64(a0: Ptr<Vec<i8>>, a1: Ptr<i8>, a2: usize) -> Ptr<Vec<i8>> {
    a0.with_mut(|__v: &mut Vec<i8>| {
        __v.pop();
        a1.with_slice(a2 as usize, |__s| __v.extend_from_slice(__s));
        __v.push(0);
    });
    a0
}

fn f66(a0: &mut Vec<i8>, a1: usize, a2: i8) -> Vec<i8> {
    a0.pop();
    a0.resize(a0.len() + (a1) as usize, a2);
    a0.push(0);
    a0.clone()
}

fn f90(a0: Ptr<Vec<i8>>, a1: usize, a2: usize, a3: Ptr<i8>, a4: usize) -> Ptr<Vec<i8>> {
    let pos = a1 as usize;
    let end = std::cmp::min(
        pos + a2 as usize,
        (*a0.upgrade().deref()).len().saturating_sub(1),
    );
    a0.with_mut(|__v: &mut Vec<i8>| {
        a3.with_slice(a4 as usize, |__s| {
            __v.splice(pos..end, __s.iter().copied());
        });
    });
    a0
}

fn f104(a0: Ptr<i8>) -> Ptr<i8> {
    a0
}

fn f105(a0: Ptr<i8>) -> Ptr<i8> {
    a0
}

fn f106(a0: Ptr<i8>) -> Ptr<i8> {
    a0
}

fn f110(a0: Vec<i8>, a1: Ptr<i8>, a2: usize) -> usize {
    let __hay = &a0[..a0.len().saturating_sub(1)];
    a1.with_c_str(|__needle| {
        (a2..=__hay.len().saturating_sub(__needle.len()))
            .find(|&__i| __hay[__i..].starts_with(__needle))
            .unwrap_or(usize::MAX)
    })
}

fn f132(a0: Vec<i8>, a1: usize, a2: usize) -> Vec<i8> {
    let mut __tmp1 =
        a0[(a1) as usize..::std::cmp::min((a1 + a2) as usize, a0.len().saturating_sub(1))].to_vec();
    __tmp1.push(0);
    __tmp1
}

fn f189(a0: Vec<i8>, a1: Ptr<i8>) -> Vec<i8> {
    let mut __tmp2 = a0;
    __tmp2.pop();
    a1.with_c_str(|__s| __tmp2.extend_from_slice(__s));
    __tmp2.push(0);
    __tmp2
}

fn f193(a0: Vec<i8>, a1: Ptr<i8>) -> Vec<i8> {
    let mut r = a0;
    r.pop();
    a1.with_c_str(|__s| r.extend_from_slice(__s));
    r.push(0);
    r
}

fn t1() -> Vec<i8> {
    vec![0]
}

fn t3() -> i8 {
    0_i8
}

fn t7<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn t8<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn t9<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn t11<T1>() -> RandomAccessIterator<T1> {
    RandomAccessIterator::default()
}

fn t12<T1>() -> RandomAccessIterator<T1> {
    RandomAccessIterator::default()
}

fn f148<T1, T2, T3>(a0: Ptr<i8>, a1: T3) -> Vec<i8> {
    let mut __bytes = a0.to_c_bytes();
    __bytes.push(0);
    __bytes
}

fn f16<T1, T2, T3>(a0: Ptr<i8>, a1: usize, a2: T3) -> Vec<i8> {
    let mut __v = Vec::with_capacity(a1 as usize + 1);
    a0.with_slice(a1 as usize, |__s| __v.extend_from_slice(__s));
    __v.push(0);
    __v
}

fn f149<T1, T2, T3>(a0: usize, a1: i8, a2: T3) -> Vec<i8> {
    vec![a1; (a0) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0))
        .collect()
}

fn f122<T1, T2, T3>(a0: Vec<i8>, a1: Ptr<i8>, a2: usize) -> usize {
    let __n = a0.len().saturating_sub(1);
    let __end = a2.saturating_add(1).min(__n);
    a1.with_c_str(|__lookup| {
        a0.iter()
            .take(__end)
            .rposition(|&__x| __lookup.contains(&__x))
            .unwrap_or(usize::MAX)
    })
}

fn f203<T1, T2, T3>(a0: Vec<i8>, a1: Ptr<i8>) -> bool {
    a1.with_c_str(|__s| a0[..a0.len().saturating_sub(1)] == *__s)
}

fn f51<T1, T2, T3>(a0: Ptr<i8>, a1: usize) -> Ptr<i8> {
    a0.offset(a1 as isize)
}
