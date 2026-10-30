// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;
use std::cell::RefCell;
use std::rc::Rc;

fn f103<T1: Clone + ByteRepr>(a0: Ptr<Vec<T1>>, a1: Vec<T1>) {
    a0.write(a1)
}

fn f104<T1: ByteRepr + Clone>(a0: Ptr<Vec<T1>>, a1: &mut Vec<T1>) {
    a0.write(std::mem::take(&mut *a1))
}

fn f108<T1>(a0: Ptr<T1>) -> RandomAccessIterator<T1> {
    a0.begin()
}

fn f109<T1>(a0: Ptr<T1>) -> RandomAccessIterator<T1> {
    a0.begin()
}

fn f110<T1>(a0: Ptr<T1>) -> RandomAccessIterator<T1> {
    a0.end()
}

fn f111<T1>(a0: Ptr<T1>) -> RandomAccessIterator<T1> {
    a0.end()
}

fn f117<T1>(a0: Ptr<T1>) -> RandomAccessIterator<T1> {
    a0.end()
}

fn f130<T1>(a0: Ptr<T1>, a1: usize) -> Ptr<T1> {
    a0.offset(a1 as isize)
}

fn f131<T1: ByteRepr + 'static>(a0: Ptr<Vec<T1>>, a1: usize) -> Ptr<T1> {
    let __idx = a1;
    if __idx as usize >= (*a0.upgrade().deref()).len() {
        panic!("out of bounds access")
    } else {
        a0.decay().offset(__idx as isize)
    }
}

fn f132<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f134<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.to_last()
}

fn f135<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0.to_last()
}

fn f136<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f137<T1>(a0: Ptr<T1>) -> Ptr<T1> {
    a0
}

fn f141<T1: ByteRepr>(a0: &mut Vec<T1>, a1: RandomAccessIterator<T1>, a2: T1) -> RandomAccessIterator<T1> {
    let __it = a1;
    let __off = __it.index();
    a0.insert(__off, a2);
    __it
}

fn f142<T1: ByteRepr>(a0: &mut Vec<T1>, a1: RandomAccessIterator<T1>, a2: T1) -> RandomAccessIterator<T1> {
    let __it = a1;
    let __off = __it.index();
    a0.insert(__off, a2);
    __it
}

fn f143<T1: ByteRepr>(a0: &mut Vec<T1>, a1: RandomAccessIterator<T1>, a2: Vec<T1>) -> RandomAccessIterator<T1> {
    let __it = a1;
    let __off = __it.index();
    a0.splice(__off..__off, a2);
    __it
}

fn f144<T1: Clone + ByteRepr>(a0: &mut Vec<T1>, a1: RandomAccessIterator<T1>, a2: usize, a3: T1) -> RandomAccessIterator<T1> {
    let __it = a1;
    let __off = __it.index();
    a0.splice(__off..__off, std::iter::repeat_n(a3, a2));
    __it
}

fn f145<T1: ByteRepr>(a0: Ptr<Vec<T1>>, a1: RandomAccessIterator<T1>) -> RandomAccessIterator<T1> {
    let __it = a1;
    let __off = __it.index();
    a0.with_mut(|__v: &mut Vec<T1>| __v.remove(__off));
    __it
}

fn t8() -> Ptr<bool> {
    Ptr::null()
}

fn t24<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn t25<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn t26<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn t27<T1>() -> Ptr<T1> {
    Ptr::null()
}

fn t28<T1>() -> RandomAccessIterator<T1> {
    RandomAccessIterator::default()
}

fn t29<T1>() -> RandomAccessIterator<T1> {
    RandomAccessIterator::default()
}

fn f128<T1, T2>(a0: Ptr<T1>, a1: usize) -> Ptr<T1> {
    a0.offset(a1 as isize)
}

fn f129<T1: ByteRepr + 'static, T2>(a0: Ptr<Vec<T1>>, a1: usize) -> Ptr<T1> {
    let __idx = a1;
    if __idx as usize >= (*a0.upgrade().deref()).len() {
        panic!("out of bounds access")
    } else {
        a0.decay().offset(__idx as isize)
    }
}

fn f149<T1: Clone + ByteRepr, T2, T3: CppInputIterator<T1>>(a0: T3, a1: T3, a2: T2) -> Vec<T1> {
    let mut __v = Vec::new();
    let mut __it = a0;
    while __it != a1 {
        __v.push(__it.iter_deref().read());
        __it.iter_inc();
    }
    __v
}

fn f155<T1: Clone + ByteRepr, T2, T3: CppInputIterator<T1>>(
    a0: Ptr<Vec<T1>>,
    a1: RandomAccessIterator<T1>,
    a2: T3,
    a3: T3,
) -> RandomAccessIterator<T1> {
    let __pos = a1;
    let __start = __pos.index();
    let mut __tmp: Vec<T1> = Vec::new();
    let mut __it = a2;
    while __it != a3 {
        __tmp.push(__it.iter_deref().read());
        __it.iter_inc();
    }
    a0.with_mut(|__v: &mut Vec<T1>| {
        __v.splice(__start..__start, __tmp);
    });
    __pos
}
