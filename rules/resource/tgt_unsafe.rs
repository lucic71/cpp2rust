// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t2() -> i32 {
    0
}

fn t3() -> i32 {
    0
}

fn t4() -> i32 {
    0
}

fn t5() -> ::libc::rlimit {
    unsafe { std::mem::zeroed() }
}

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

unsafe fn f4(a0: i32, a1: *mut ::libc::rlimit) -> i32 {
    libc::getrlimit(a0 as u32, a1)
}

unsafe fn f5(a0: i32, a1: *const ::libc::rlimit) -> i32 {
    libc::setrlimit(a0 as u32, a1)
}

unsafe fn f6() -> i32 {
    ::libc::RLIMIT_STACK as i32
}

unsafe fn f7() -> i32 {
    ::libc::RLIMIT_DATA as i32
}

unsafe fn f8() -> i32 {
    ::libc::RLIMIT_NOFILE as i32
}

unsafe fn f9() -> i32 {
    ::libc::RLIMIT_CORE as i32
}
