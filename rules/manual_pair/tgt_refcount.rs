// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f15<T1, T2>(a0: (Value<T1>, Value<T2>)) -> Value<T2> {
    a0.1
}

fn f16<T1, T2>(a0: (Value<T1>, Value<T2>)) -> Value<T1> {
    a0.0
}
