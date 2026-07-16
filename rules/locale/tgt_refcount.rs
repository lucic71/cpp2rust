// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1(a0: i32, a1: Ptr<u8>) -> Ptr<u8> {
    Ptr::from_string_literal(b"C")
}
