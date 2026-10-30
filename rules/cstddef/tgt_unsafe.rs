// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

fn f4(a0: &mut u8, a1: u32) -> u8 {
    let n_ = *a0 << a1;
    *a0 = n_;
    *a0
}

fn f5(a0: &mut u8, a1: u32) -> u8 {
    *a0 >> a1
}

fn f6(a0: &mut u8, a1: u32) -> u8 {
    let n_ = *a0 >> a1;
    *a0 = n_;
    *a0
}

fn t1() -> u8 {
    Default::default()
}

fn t1002() -> usize {
    0_usize
}

fn t1003() -> isize {
    0_isize
}

fn t1005() -> usize {
    0_usize
}

fn t1006() -> isize {
    0_isize
}
