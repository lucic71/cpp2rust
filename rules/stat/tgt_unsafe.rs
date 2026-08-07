// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn t1() -> libcc2rs::Stat {
    Default::default()
}

unsafe fn f1(a0: *const libc::c_char, a1: *mut libcc2rs::Stat) -> i32 {
    libcc2rs::stat_unsafe(a0, a1 as *mut ::libc::stat)
}

unsafe fn f2(a0: i32, a1: *mut libcc2rs::Stat) -> i32 {
    libcc2rs::fstat_unsafe(a0, a1 as *mut ::libc::stat)
}

unsafe fn f3(a0: *const libc::c_char, a1: ::libc::mode_t) -> i32 {
    libcc2rs::mkdir_unsafe(a0, a1 as ::libc::mode_t)
}

unsafe fn f4(a0: *const libc::c_char, a1: ::libc::mode_t) -> i32 {
    libc::chmod(a0, a1 as ::libc::mode_t)
}

unsafe fn f5(a0: i32, a1: *const libc::c_char, a2: *const libcc2rs::Timespec, a3: i32) -> i32 {
    libc::utimensat(a0, a1, a2 as *const ::libc::timespec, a3)
}

unsafe fn f6(a0: *const libc::c_char, a1: *mut libcc2rs::Stat) -> i32 {
    libcc2rs::lstat_unsafe(a0, a1 as *mut ::libc::stat)
}

unsafe fn f7(a0: i32, a1: ::libc::mode_t) -> i32 {
    libcc2rs::fchmod_unsafe(a0, a1)
}
