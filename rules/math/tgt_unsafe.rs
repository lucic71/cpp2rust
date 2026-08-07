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
