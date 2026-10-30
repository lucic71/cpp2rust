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
