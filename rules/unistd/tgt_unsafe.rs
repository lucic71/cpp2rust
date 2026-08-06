// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: i32) -> i32 {
    libcc2rs::close_unsafe(a0)
}

unsafe fn f2(a0: i32, a1: i64, a2: i32) -> i64 {
    libc::lseek(a0, a1, a2)
}

unsafe fn f3(a0: i32, a1: *mut ::libc::c_void, a2: usize) -> isize {
    libcc2rs::read_unsafe(a0, a1, a2)
}

unsafe fn f4(a0: *const libc::c_char) -> i32 {
    libcc2rs::unlink_unsafe(a0)
}

unsafe fn f5(a0: *mut i32) -> i32 {
    libc::pipe(a0)
}

unsafe fn f6(a0: i32, a1: i64) -> i32 {
    libcc2rs::ftruncate_unsafe(a0, a1)
}

unsafe fn f7(a0: i32) -> i32 {
    libc::isatty(a0)
}

unsafe fn f8() -> u32 {
    libcc2rs::geteuid_unsafe()
}

unsafe fn f9(a0: *mut libc::c_char, a1: usize) -> i32 {
    libc::gethostname(a0, a1)
}

unsafe fn f10(a0: i32, a1: *const ::libc::c_void, a2: usize) -> isize {
    libcc2rs::write_unsafe(a0, a1, a2)
}

unsafe fn f11(a0: *const libc::c_char) -> i32 {
    libcc2rs::rmdir_unsafe(a0)
}

unsafe fn f12(a0: *const libc::c_char, a1: ::libc::uid_t, a2: ::libc::gid_t) -> i32 {
    libc::chown(a0, a1, a2)
}

unsafe fn f13(a0: *const libc::c_char, a1: i32) -> i32 {
    libc::access(a0, a1)
}

unsafe fn f14(a0: *const libc::c_char, a1: *mut libc::c_char, a2: usize) -> isize {
    libc::readlink(a0, a1, a2)
}

unsafe fn f15(a0: *const libc::c_char, a1: *const libc::c_char) -> i32 {
    libc::symlink(a0, a1)
}

unsafe fn f16(a0: *mut libc::c_char, a1: usize) -> *mut libc::c_char {
    libc::getcwd(a0, a1)
}

unsafe fn f17(a0: *const libc::c_char) -> i32 {
    libc::chdir(a0)
}

unsafe fn f18(a0: i32) -> i32 {
    libc::fsync(a0)
}

unsafe fn f19(a0: i32, a1: *mut ::libc::c_void, a2: usize, a3: ::libc::off_t) -> isize {
    libc::pread(a0, a1, a2, a3)
}

unsafe fn f20(a0: i32, a1: *const ::libc::c_void, a2: usize, a3: ::libc::off_t) -> isize {
    libc::pwrite(a0, a1, a2, a3)
}

unsafe fn f21() -> ::libc::pid_t {
    libc::getpid()
}

unsafe fn f22() -> ::libc::uid_t {
    libc::getuid()
}

unsafe fn f23(a0: i32, a1: ::libc::uid_t, a2: ::libc::gid_t) -> i32 {
    libc::fchown(a0, a1, a2)
}
