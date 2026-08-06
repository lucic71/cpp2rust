// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t1() -> ::libc::rusage {
    unsafe { std::mem::zeroed() }
}

unsafe fn f1(a0: i32, a1: *mut ::libc::rusage) -> i32 {
    libc::getrusage(a0, a1)
}

unsafe fn f2() -> i32 {
    libc::RUSAGE_SELF
}

unsafe fn f3() -> i32 {
    libc::RUSAGE_CHILDREN
}
