// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

unsafe fn f4<T1>(a0: Vec<T1>) -> usize {
    a0.len()
}
