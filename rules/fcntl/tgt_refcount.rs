// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1(a0: i32, a1: i32, va: &[VaArg]) -> i32 {
    panic!(
        "fcntl is not supported in the refcount model (fd={}, cmd={}, varargs={})",
        a0,
        a1,
        va.len()
    )
}

fn f2(a0: Ptr<u8>, a1: i32, va: &[VaArg]) -> i32 {
    panic!(
        "open is not supported in the refcount model (path={:?}, flags={}, varargs={})",
        a0.to_rust_string(),
        a1,
        va.len()
    )
}
