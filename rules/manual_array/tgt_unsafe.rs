// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

unsafe fn f4<T1>(a0: &mut Vec<T1>) -> Vec<T1> {
    std::mem::take(&mut *a0)
}

unsafe fn f5<T1>(a0: &mut Vec<T1>, a1: &mut Vec<T1>) {
    *a0 = std::mem::take(&mut *a1)
}

unsafe fn f6<T1: Clone>(a0: Vec<T1>) -> Vec<T1> {
    a0.clone()
}

unsafe fn f7<T1: Clone>(a0: &mut Vec<T1>, a1: Vec<T1>) {
    *a0 = a1.clone()
}
