// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

unsafe fn f1<T1>() -> Vec<T1> {
    Vec::new()
}

unsafe fn f5<T1: Clone>(a0: Vec<T1>) -> Vec<T1> {
    a0.clone()
}

unsafe fn f6<T1>(a0: &mut Vec<T1>) -> Vec<T1> {
    std::mem::take(&mut *a0)
}

unsafe fn f10<T1: Clone>(a0: &mut Vec<T1>, a1: Vec<T1>) {
    *a0 = a1.clone()
}

unsafe fn f11<T1>(a0: &mut Vec<T1>, a1: &mut Vec<T1>) {
    *a0 = std::mem::take(&mut *a1)
}

unsafe fn f33<T1>(a0: Vec<T1>) -> bool {
    a0.is_empty()
}

unsafe fn f38<T1>(a0: &mut Vec<T1>) -> *const T1 {
    ((a0).first_mut().unwrap())
}

unsafe fn f40<T1>(a0: &mut Vec<T1>) -> *const T1 {
    (a0.last_mut().unwrap())
}

unsafe fn f44<T1>(a0: &mut Vec<T1>, a1: T1) {
    a0.push(a1)
}

unsafe fn f45<T1: Default>(a0: &mut Vec<T1>, a1: &mut T1) {
    a0.push(std::mem::take(&mut *a1))
}

unsafe fn f46<T1>(a0: &mut Vec<T1>) -> T1 {
    a0.remove(0)
}

unsafe fn f61<T1>(a0: &mut Vec<T1>, a1: T1) {
    let __init = a1;
    a0.push(__init)
}
