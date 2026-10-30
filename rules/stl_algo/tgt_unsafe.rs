// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::io::{Seek, Write};

unsafe fn f42<T1: UnsafeCppInputIterator<T2>, T2: PartialOrd>(a0: T1, a1: T1) -> T1 {
    let __first = a0;
    let __last = a1;
    let mut __best = __first.clone();
    if __first != __last {
        let mut __it = __first;
        __it.iter_inc();
        while __it != __last {
            if *__best.iter_deref() < *__it.iter_deref() {
                __best = __it.clone();
            }
            __it.iter_inc();
        }
    }
    __best
}

unsafe fn f70<T1: UnsafeCppRandomAccessIterator<T2>, T2>(a0: T1, a1: T1) {
    let __first = a0;
    let __n = a1.iter_distance(&__first);
    for __i in (1..__n).rev() {
        let __j = libc::rand() as isize % (__i + 1);
        cpp_swap_unsafe!(__first.iter_at(__i), __first.iter_at(__j));
    }
}

unsafe fn f97<T1: UnsafeCppRandomAccessIterator<T2>, T2: Ord>(a0: T1, a1: T1) {
    let __first = a0;
    let __n = a1.iter_distance(&__first);
    let mut __order: Vec<isize> = (0..__n).collect();
    __order.sort_by(|&__x, &__y| (*__first.iter_at(__x)).cmp(&*__first.iter_at(__y)));
    let __values: Vec<_> = __order
        .iter()
        .map(|&__i| std::ptr::read(__first.iter_at(__i)))
        .collect();
    for (__i, __v) in (0..__n).zip(__values) {
        std::ptr::write(__first.iter_at(__i), __v);
    }
}

unsafe fn f104<T1: UnsafeCppInputIterator<T2>, T2: PartialEq + Clone>(a0: T1, a1: T1) -> T1 {
    let __first = a0;
    let __last = a1;
    if __first == __last {
        __last
    } else {
        let mut __write = __first.clone();
        let mut __it = __first;
        __it.iter_inc();
        while __it != __last {
            if *__write.iter_deref() != *__it.iter_deref() {
                __write.iter_inc();
                *__write.iter_deref() = (*__it.iter_deref()).clone();
            }
            __it.iter_inc();
        }
        __write.iter_inc();
        __write
    }
}
