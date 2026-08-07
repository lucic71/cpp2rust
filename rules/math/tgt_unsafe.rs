// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: f64) -> f64 {
    a0.cos()
}
unsafe fn f2(a0: f64) -> f64 {
    a0.round()
}
unsafe fn f3(a0: f64) -> f64 {
    a0.sin()
}
unsafe fn f4(a0: f64) -> f64 {
    a0.abs()
}
unsafe fn f5(a0: f64) -> f64 {
    a0.trunc()
}
unsafe fn f6(a0: f64) -> f64 {
    a0.floor()
}
unsafe fn f7(a0: f64, a1: f64) -> f64 {
    a0 % a1
}
unsafe fn f8(a0: f64) -> f64 {
    a0.ceil()
}
unsafe fn f9(a0: f64) -> f64 {
    a0.sqrt()
}
unsafe fn f10(a0: f64) -> f64 {
    a0.sin()
}
unsafe fn f11(a0: f64) -> f64 {
    a0.tan()
}
unsafe fn f12(a0: f64) -> f64 {
    a0.asin()
}
unsafe fn f13(a0: f64) -> f64 {
    a0.acos()
}
unsafe fn f14(a0: f64) -> f64 {
    a0.atan()
}
unsafe fn f15(a0: f64) -> f64 {
    a0.sinh()
}
unsafe fn f16(a0: f64) -> f64 {
    a0.cosh()
}
unsafe fn f17(a0: f64) -> f64 {
    a0.tanh()
}
unsafe fn f18(a0: f64) -> f64 {
    a0.asinh()
}
unsafe fn f19(a0: f64) -> f64 {
    a0.acosh()
}
unsafe fn f20(a0: f64) -> f64 {
    a0.atanh()
}
unsafe fn f21(a0: f64) -> f64 {
    a0.exp()
}
unsafe fn f22(a0: f64) -> f64 {
    a0.exp_m1()
}
unsafe fn f23(a0: f64) -> f64 {
    a0.ln()
}
unsafe fn f24(a0: f64) -> f64 {
    a0.log10()
}
unsafe fn f25(a0: f64) -> f64 {
    a0.ln_1p()
}
unsafe fn f26(a0: f64) -> f64 {
    a0.cbrt()
}
unsafe fn f27(a0: f64) -> i64 {
    a0.round_ties_even() as i64
}
unsafe fn f28(a0: f64, a1: f64) -> f64 {
    a0.atan2(a1)
}
unsafe fn f29(a0: f64, a1: f64) -> f64 {
    a0.hypot(a1)
}
unsafe fn f30(a0: f64, a1: f64) -> f64 {
    a0.copysign(a1)
}
unsafe fn f31(a0: f64, a1: f64) -> f64 {
    a0.powf(a1)
}
unsafe fn f32(a0: f64, a1: i32) -> f64 {
    a0 * (2.0_f64).powi(a1)
}
unsafe fn f33(a0: f64, a1: *mut i32) -> f64 {
    match a0 == 0.0 || !a0.is_finite() {
        true => {
            *a1 = 0;
            a0
        }
        false => {
            let __e = a0.abs().log2().floor() as i32 + 1;
            *a1 = __e;
            a0 / (2.0_f64).powi(__e)
        }
    }
}
