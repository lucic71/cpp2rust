// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

fn f42<T1: CppInputIterator<T2>, T2: CppLess>(a0: T1, a1: T1) -> T1 {
    let __first = a0;
    let __last = a1;
    let mut __best = __first.clone();
    if __first != __last {
        let mut __it = __first;
        __it.iter_inc();
        while __it != __last {
            if CppLess::less(&__best.iter_deref(), &__it.iter_deref()) {
                __best = __it.clone();
            }
            __it.iter_inc();
        }
    }
    __best
}

fn f70<T1: CppRandomAccessIterator<T2>, T2: Clone + ByteRepr>(a0: T1, a1: T1) {
    let __first = a0;
    let __n = a1.iter_distance(&__first);
    let __random = std::hash::RandomState::new();
    for __i in (1..__n).rev() {
        let __j = (std::hash::BuildHasher::hash_one(&__random, __i) % (__i + 1) as u64) as isize;
        cpp_swap!(__first.iter_at(__i), __first.iter_at(__j));
    }
}

fn f97<T1: CppRandomAccessIterator<T2>, T2: CppLess + Clone + ByteRepr>(a0: T1, a1: T1) {
    let __first = a0;
    let __n = a1.iter_distance(&__first);
    let mut __order: Vec<isize> = (0..__n).collect();
    __order.sort_by(|&__x, &__y| cpp_cmp(&__first.iter_at(__x), &__first.iter_at(__y)));
    reorder(__order, |__i| __first.iter_at(__i));
}

fn f104<T1: CppInputIterator<T2>, T2: CppEq + Clone + ByteRepr>(a0: T1, a1: T1) -> T1 {
    let __first = a0;
    let __last = a1;
    if __first == __last {
        __last
    } else {
        let mut __write = __first.clone();
        let mut __it = __first;
        __it.iter_inc();
        while __it != __last {
            if !CppEq::equal(&__write.iter_deref(), &__it.iter_deref()) {
                __write.iter_inc();
                __write.iter_deref().write(__it.iter_deref().read());
            }
            __it.iter_inc();
        }
        __write.iter_inc();
        __write
    }
}

fn f16<T1: CppInputIterator<T3>, T2, T3: CppEq<T2>>(a0: T1, a1: T1, a2: Ptr<T2>) -> T1 {
    let mut __it = a0;
    let __last = a1;
    let __value = a2;
    while __it != __last && !CppEq::equal(&__it.iter_deref(), &__value) {
        __it.iter_inc();
    }
    __it
}

fn f101<T1: CppRandomAccessIterator<T3>, T2, T3: Clone + ByteRepr>(a0: T1, a1: T1, a2: T2)
where
    T2: Callable2<Ptr<T3>, Ptr<T3>, bool>,
{
    let __first = a0;
    let __n = a1.iter_distance(&__first);
    let __cmp = a2;
    let mut __order: Vec<isize> = (0..__n).collect();
    __order.sort_by(|&__x, &__y| {
        let __a = __first.iter_at(__x);
        let __b = __first.iter_at(__y);
        if __cmp.call(__a.clone(), __b.clone()) {
            std::cmp::Ordering::Less
        } else if __cmp.call(__b, __a) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    reorder(__order, |__i| __first.iter_at(__i));
}
