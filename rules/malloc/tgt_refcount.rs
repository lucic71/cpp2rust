// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1(a0: AnyPtr) -> usize {
    a0.reinterpret_cast::<u8>().len()
}
