// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: usize, a1: usize) -> usize {
    a0
}
unsafe fn f2(a0: u32) -> i32 {
    a0.trailing_zeros() as i32
}
unsafe fn f3(a0: u32) -> i32 {
    a0.leading_zeros() as i32
}
unsafe fn f4(a0: u16) -> u16 {
    a0.swap_bytes()
}
unsafe fn f5(a0: u32) -> u32 {
    a0.swap_bytes()
}
unsafe fn f6(a0: u64) -> u64 {
    a0.swap_bytes()
}
unsafe fn f7(a0: u64) -> i32 {
    a0.trailing_zeros() as i32
}
unsafe fn f8(a0: u64) -> i32 {
    a0.count_ones() as i32
}
unsafe fn f9(a0: i64, a1: i64, a2: *mut i64) -> bool {
    let (val, ovf) = a0.overflowing_mul(a1);
    *a2 = val;
    ovf
}
unsafe fn f10(a0: i64, a1: i64, a2: *mut i64) -> bool {
    let (val, ovf) = a0.overflowing_mul(a1);
    *a2 = val;
    ovf
}
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
unsafe fn f11() {
    std::hint::spin_loop();
}

unsafe fn f12(a0: i64, a1: i64, a2: *mut i64) -> bool {
    let (val, ovf) = a0.overflowing_mul(a1);
    *a2 = val;
    ovf
}
unsafe fn f13(a0: i64, a1: i64, a2: *mut i64) -> bool {
    let (val, ovf) = a0.overflowing_mul(a1);
    *a2 = val;
    ovf
}
unsafe fn f14(a0: u64) -> u64 {
    a0.swap_bytes()
}

unsafe fn f15(a0: u64) -> i32 {
    a0.trailing_zeros() as i32
}

unsafe fn f16() -> f64 {
    f64::INFINITY
}

unsafe fn f17(a0: *const ::libc::c_void, va: &[::libcc2rs::VaArg]) {
    ();
}

unsafe fn f18(a0: f64) -> f64 {
    a0.ceil()
}

unsafe fn f19(a0: f64) -> f64 {
    a0.floor()
}

unsafe fn f20(a0: u64) -> i32 {
    a0.leading_zeros() as i32
}

unsafe fn f21() -> f32 {
    f32::INFINITY
}

unsafe fn f22(a0: *const libc::c_char) -> f32 {
    f32::NAN
}

unsafe fn f23() {
    ::std::unreachable!()
}

unsafe fn f24(a0: i32, a1: i32, a2: *mut usize) -> bool {
    let __prod = (a0 as i64) * (a1 as i64);
    *a2 = __prod as usize;
    usize::try_from(__prod).is_err()
}
