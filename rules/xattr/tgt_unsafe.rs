// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(
    a0: i32,
    a1: *const u8,
    a2: *const ::libc::c_void,
    a3: u64,
    a4: i32,
) -> i32 {
    libc::fsetxattr(a0, a1 as *const i8, a2, a3 as usize, a4)
}

unsafe fn f2(a0: i32, a1: *const u8, a2: *mut ::libc::c_void, a3: u64) -> i64 {
    libc::fgetxattr(a0, a1 as *const i8, a2, a3 as usize) as i64
}
