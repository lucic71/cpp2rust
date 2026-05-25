// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: u32) -> u32 {
    u32::from_be(a0)
}
unsafe fn f2(a0: u16) -> u16 {
    u16::from_be(a0)
}
unsafe fn f3(a0: u16) -> u16 {
    u16::to_be(a0)
}
unsafe fn f4(a0: u32) -> u32 {
    u32::to_be(a0)
}
unsafe fn f5(a0: i32, a1: *const u8, a2: *mut ::libc::c_void) -> i32 {
    unsafe extern "C" {
        fn inet_pton(af: i32, src: *const u8, dst: *mut ::libc::c_void) -> i32;
    }
    inet_pton(a0, a1, a2)
}
unsafe fn f6(a0: i32, a1: *const ::libc::c_void, a2: *mut u8, a3: u32) -> *const u8 {
    unsafe extern "C" {
        fn inet_ntop(af: i32, src: *const ::libc::c_void, dst: *mut u8, size: u32) -> *const u8;
    }
    inet_ntop(a0, a1, a2, a3)
}
