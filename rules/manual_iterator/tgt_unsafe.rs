// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

unsafe fn f1<T1: UnsafeCppIterator<T2>, T2>(a0: T1) -> *mut T2 {
    a0.iter_deref()
}
