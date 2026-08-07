// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f12(a0: i64, a1: i64, a2: Ptr<i64>) -> bool {
    let __wide = (a0 as i128).wrapping_mul(a1 as i128);
    a2.clone().write(__wide as _);
    (a0 as i128).checked_mul(a1 as i128).is_none() || (a2.read() as i128) != __wide
}

fn f17(a0: AnyPtr, va: &[VaArg]) {
    ();
}

fn f25(a0: f64) -> i32 {
    a0.is_nan() as i32
}

fn f26(a0: f64) -> i32 {
    a0.is_finite() as i32
}

fn f27(a0: f64) -> i32 {
    a0.is_sign_negative() as i32
}

fn f28(a0: f64) -> i32 {
    match a0.is_infinite() {
        true => match a0.is_sign_negative() {
            true => -1,
            false => 1,
        },
        false => 0,
    }
}

fn f29(a0: u32) -> AnyPtr {
    AnyPtr::from_int(usize::MAX / 2)
}
