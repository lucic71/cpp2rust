// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f33(a0: f64, a1: Ptr<i32>) -> f64 {
    match a0 == 0.0 || !a0.is_finite() {
        true => {
            a1.clone().write(0);
            a0
        }
        false => {
            let __e = a0.abs().log2().floor() as i32 + 1;
            a1.clone().write(__e);
            a0 / (2.0_f64).powi(__e)
        }
    }
}
