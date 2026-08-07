// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1(a0: i32, a1: Ptr<i32>, a2: i32) -> i32 {
    panic!("waitpid: child processes are not supported in the refcount model")
}

fn f2() -> i32 {
    ::libc::WNOHANG
}

fn f3() -> i32 {
    ::libc::WUNTRACED
}
