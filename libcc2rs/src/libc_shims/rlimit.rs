// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::ByteRepr;

#[derive(Default, Clone)]
pub struct Rlimit {
    pub rlim_cur: u64,
    pub rlim_max: u64,
}

impl Rlimit {
    pub fn from_libc(r: &::libc::rlimit) -> Self {
        Self {
            rlim_cur: r.rlim_cur,
            rlim_max: r.rlim_max,
        }
    }

    pub fn to_libc(&self) -> ::libc::rlimit {
        ::libc::rlimit {
            rlim_cur: self.rlim_cur,
            rlim_max: self.rlim_max,
        }
    }
}

impl ByteRepr for Rlimit {
    fn byte_size() -> usize {
        16
    }

    fn to_bytes(&self, buf: &mut [u8]) {
        self.rlim_cur.to_bytes(&mut buf[0..8]);
        self.rlim_max.to_bytes(&mut buf[8..16]);
    }

    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            rlim_cur: u64::from_bytes(&buf[0..8]),
            rlim_max: u64::from_bytes(&buf[8..16]),
        }
    }
}

impl ByteRepr for ::libc::rlimit {}
