// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: i32, a1: *mut i32, a2: i32) -> i32 {
    libc::waitpid(a0, a1, a2)
}

unsafe fn f2() -> i32 {
    ::libc::WNOHANG
}

unsafe fn f3() -> i32 {
    ::libc::WUNTRACED
}
