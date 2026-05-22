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
