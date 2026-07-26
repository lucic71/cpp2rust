// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::ByteRepr;

#[derive(Default, Clone)]
pub struct Pollfd {
    pub fd: i32,
    pub events: i16,
    pub revents: i16,
}

impl ByteRepr for Pollfd {}

impl ByteRepr for ::libc::pollfd {}
