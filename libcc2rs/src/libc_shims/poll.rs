// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::ByteRepr;

#[derive(Default, Clone)]
pub struct Pollfd {
    pub fd: i32,
    pub events: i16,
    pub revents: i16,
}

impl ByteRepr for Pollfd {
    fn byte_size() -> usize {
        8
    }

    fn to_bytes(&self, buf: &mut [u8]) {
        self.fd.to_bytes(&mut buf[0..4]);
        self.events.to_bytes(&mut buf[4..6]);
        self.revents.to_bytes(&mut buf[6..8]);
    }

    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            fd: i32::from_bytes(&buf[0..4]),
            events: i16::from_bytes(&buf[4..6]),
            revents: i16::from_bytes(&buf[6..8]),
        }
    }
}

impl ByteRepr for ::libc::pollfd {}
