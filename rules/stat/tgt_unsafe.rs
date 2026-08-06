// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t1() -> ::libc::stat {
    unsafe { std::mem::zeroed() }
}

unsafe fn f1(a0: *const libc::c_char, a1: *mut ::libc::stat) -> i32 {
    libcc2rs::stat_unsafe(a0, a1)
}

unsafe fn f2(a0: i32, a1: *mut ::libc::stat) -> i32 {
    libcc2rs::fstat_unsafe(a0, a1)
}

unsafe fn f3(a0: *const libc::c_char, a1: ::libc::mode_t) -> i32 {
    libcc2rs::mkdir_unsafe(a0, a1 as ::libc::mode_t)
}

unsafe fn f4(a0: *const libc::c_char, a1: ::libc::mode_t) -> i32 {
    libc::chmod(a0, a1 as ::libc::mode_t)
}

unsafe fn f5(a0: i32, a1: *const libc::c_char, a2: *const ::libc::timespec, a3: i32) -> i32 {
    libc::utimensat(a0, a1, a2, a3)
}

unsafe fn f6(a0: *const libc::c_char, a1: *mut ::libc::stat) -> i32 {
    libcc2rs::lstat_unsafe(a0, a1)
}

unsafe fn f7(a0: i32, a1: ::libc::mode_t) -> i32 {
    libcc2rs::fchmod_unsafe(a0, a1)
}
