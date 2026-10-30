// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

unsafe fn f1<T1, const T2: usize>(a0: [T1; T2]) -> Vec<T1> {
    Vec::from(a0)
}

unsafe fn f2<T1, const T2: usize>(a0: [T1; T2]) -> Vec<T1> {
    Vec::from(a0)
}
