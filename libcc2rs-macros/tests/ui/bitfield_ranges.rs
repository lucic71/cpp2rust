// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs_macros::bitfields;

// `b` starts inside `a`
#[bitfields(__bits_0 { a: i32 @ 0..4 unsigned, b: i32 @ 2..6 unsigned })]
pub struct overlapping {
    pub __bits_0: [u8; 1],
}

// `wide` needs 3 bytes of storage
#[bitfields(__bits_0 { wide: i32 @ 0..20 unsigned })]
pub struct too_narrow {
    pub __bits_0: [u8; 1],
}

// no such field
#[bitfields(__bits_1 { a: i32 @ 0..1 unsigned })]
pub struct wrong_storage {
    pub __bits_0: [u8; 1],
}

fn main() {}
