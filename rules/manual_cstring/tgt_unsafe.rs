// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#[cfg(target_os = "linux")]
unsafe fn f25(a0: *const u8, a1: i32, a2: usize) -> *const ::libc::c_void {
    libc::memrchr(a0 as *const ::libc::c_void, a1, a2 as usize) as *const ::libc::c_void
}

#[cfg(target_os = "linux")]
unsafe fn f26(a0: *mut u8, a1: i32, a2: usize) -> *mut ::libc::c_void {
    libc::memrchr(a0 as *const ::libc::c_void, a1, a2 as usize)
}
