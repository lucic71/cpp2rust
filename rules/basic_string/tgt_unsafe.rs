// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::io::Read;

unsafe fn f10() -> Vec<libc::c_char> {
    vec![0]
}

unsafe fn f12(a0: Vec<libc::c_char>) -> Vec<libc::c_char> {
    a0.clone()
}

unsafe fn f17(a0: &mut Vec<libc::c_char>) -> Vec<libc::c_char> {
    std::mem::take(&mut *a0)
}

unsafe fn f21(a0: &mut Vec<libc::c_char>, a1: Vec<libc::c_char>) {
    *a0 = a1.clone()
}

unsafe fn f24(a0: &mut Vec<libc::c_char>, a1: &mut Vec<libc::c_char>) {
    *a0 = std::mem::take(&mut *a1)
}

unsafe fn f27(a0: &mut Vec<libc::c_char>) -> UnsafeRandomAccessIterator<libc::c_char> {
    UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr())
}

unsafe fn f29(a0: &mut Vec<libc::c_char>) -> UnsafeRandomAccessIterator<libc::c_char> {
    UnsafeRandomAccessIterator::Vec(a0.as_mut_ptr().add(a0.len() - 1))
}

unsafe fn f39(a0: Vec<libc::c_char>) -> usize {
    (a0.len() - 1)
}

unsafe fn f40(a0: Vec<libc::c_char>) -> usize {
    (a0.len() - 1)
}

unsafe fn f43(a0: &mut Vec<libc::c_char>, a1: usize) {
    a0.pop();
    a0.resize((a1) as usize, 0);
    a0.push(0)
}

unsafe fn f44(a0: &mut Vec<libc::c_char>) {
    a0.shrink_to_fit()
}

unsafe fn f48(a0: &mut Vec<libc::c_char>) {
    a0.clear();
    a0.push(0)
}

unsafe fn f49(a0: Vec<libc::c_char>) -> bool {
    a0.len() <= 1
}

unsafe fn f53(a0: &mut Vec<libc::c_char>, a1: usize) -> *mut libc::c_char {
    if a1 as usize >= a0.len() - 1 {
        panic!("out of bounds access")
    } else {
        (&mut a0[a1 as usize] as *mut libc::c_char)
    }
}

unsafe fn f64(a0: &mut Vec<libc::c_char>, a1: *mut libc::c_char, a2: usize) {
    a0.splice(a0.len().saturating_sub(1)..a0.len(), {
        let mut v = ::std::slice::from_raw_parts(a1, a2 as usize).to_vec();
        v.push(0);
        v
    });
}

unsafe fn f66(a0: &mut Vec<libc::c_char>, a1: usize, a2: libc::c_char) {
    a0.splice(
        a0.len() - 1..a0.len() - 1,
        ::std::vec::from_elem(a2, a1 as usize),
    );
}

unsafe fn f90(a0: &mut Vec<libc::c_char>, a1: usize, a2: usize, a3: *const libc::c_char, a4: usize) {
    a0.splice(
        a1 as usize..a1 as usize + a2 as usize,
        ::std::slice::from_raw_parts(a3, a4 as usize).to_vec(),
    );
}

unsafe fn f104(a0: Vec<libc::c_char>) -> *const libc::c_char {
    a0.as_ptr()
}

unsafe fn f105(a0: Vec<libc::c_char>) -> *const libc::c_char {
    a0.as_ptr()
}

unsafe fn f106(a0: &mut Vec<libc::c_char>) -> *const libc::c_char {
    a0.as_mut_ptr()
}

unsafe fn f110(a0: Vec<libc::c_char>, a1: *const libc::c_char, a2: usize) -> usize {
    let __hay = &a0[..a0.len() - 1];
    let __needle = ::std::slice::from_raw_parts(a1, libc::strlen(a1));
    (a2..=__hay.len().saturating_sub(__needle.len()))
        .find(|&__i| __hay[__i..].starts_with(__needle))
        .unwrap_or(usize::MAX)
}

unsafe fn f132(a0: Vec<libc::c_char>, a1: usize, a2: usize) -> Vec<libc::c_char> {
    let mut __tmp1 = a0[(a1) as usize..::std::cmp::min((a1 + a2) as usize, a0.len() - 1)].to_vec();
    __tmp1.push(0);
    __tmp1
}

unsafe fn f189(a0: Vec<libc::c_char>, a1: *const libc::c_char) -> Vec<libc::c_char> {
    let mut __tmp2 = a0.clone();
    __tmp2.pop();
    let __from = a1;
    __tmp2.extend_from_slice(::std::slice::from_raw_parts(
        __from,
        (0..).position(|i| *__from.add(i) == 0).unwrap(),
    ));
    __tmp2.push(0);
    __tmp2
}

unsafe fn f193(a0: Vec<libc::c_char>, a1: *const libc::c_char) -> Vec<libc::c_char> {
    let mut __tmp2 = a0.clone();
    __tmp2.pop();
    let __from = a1;
    __tmp2.extend_from_slice(::std::slice::from_raw_parts(
        __from,
        (0..).position(|i| *__from.add(i) == 0).unwrap(),
    ));
    __tmp2.push(0);
    __tmp2
}

unsafe fn f203(a0: Vec<libc::c_char>, a1: *const libc::c_char) -> bool {
    a0 == {
        let s = a1;
        std::slice::from_raw_parts(s, (0..).take_while(|&i| *s.add(i) != 0).count() + 1).to_vec()
    }
}

fn t3() -> libc::c_char {
    0 as libc::c_char
}

fn t5() -> usize {
    0_usize
}
