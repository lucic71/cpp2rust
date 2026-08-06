// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::ByteRepr;

#[derive(Default, Clone)]
pub struct Flock {
    pub l_type: i16,
    pub l_whence: i16,
    pub l_start: i64,
    pub l_len: i64,
    pub l_pid: i32,
}

impl Flock {
    pub fn from_libc(f: &::libc::flock) -> Self {
        Self {
            l_type: f.l_type,
            l_whence: f.l_whence,
            l_start: f.l_start,
            l_len: f.l_len,
            l_pid: f.l_pid,
        }
    }

    pub fn to_libc(&self) -> ::libc::flock {
        let mut f: ::libc::flock = unsafe { std::mem::zeroed() };
        f.l_type = self.l_type;
        f.l_whence = self.l_whence;
        f.l_start = self.l_start;
        f.l_len = self.l_len;
        f.l_pid = self.l_pid;
        f
    }
}

impl ByteRepr for Flock {
    fn byte_size() -> usize {
        32
    }

    fn to_bytes(&self, buf: &mut [u8]) {
        self.l_type.to_bytes(&mut buf[0..2]);
        self.l_whence.to_bytes(&mut buf[2..4]);
        self.l_start.to_bytes(&mut buf[8..16]);
        self.l_len.to_bytes(&mut buf[16..24]);
        self.l_pid.to_bytes(&mut buf[24..28]);
    }

    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            l_type: i16::from_bytes(&buf[0..2]),
            l_whence: i16::from_bytes(&buf[2..4]),
            l_start: i64::from_bytes(&buf[8..16]),
            l_len: i64::from_bytes(&buf[16..24]),
            l_pid: i32::from_bytes(&buf[24..28]),
        }
    }
}

impl ByteRepr for ::libc::flock {}
