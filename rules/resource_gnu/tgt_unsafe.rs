// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: i32, a1: *mut ::libc::rusage) -> i32 {
    libc::getrusage(a0, a1)
}

unsafe fn f2(a0: i32, a1: *mut ::libc::rlimit) -> i32 {
    libc::getrlimit(a0 as u32, a1)
}

unsafe fn f3(a0: i32, a1: *const ::libc::rlimit) -> i32 {
    libc::setrlimit(a0 as u32, a1)
}
