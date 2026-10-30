// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1<T1: From<T2>, T2>(a0: T2) -> T1 {
    <T1>::from(a0)
}
