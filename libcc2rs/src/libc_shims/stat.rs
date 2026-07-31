// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::ByteRepr;

#[derive(Default, Clone)]
pub struct Stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atime: i64,
    pub st_mtime: i64,
    pub st_ctime: i64,
}

impl Stat {
    #[allow(clippy::unnecessary_cast)]
    pub fn from_libc(s: &::libc::stat) -> Self {
        Self {
            st_dev: s.st_dev as u64,
            st_ino: s.st_ino as u64,
            st_nlink: s.st_nlink as u64,
            st_mode: s.st_mode as u32,
            st_uid: s.st_uid,
            st_gid: s.st_gid,
            st_rdev: s.st_rdev as u64,
            st_size: s.st_size as i64,
            st_blksize: s.st_blksize as i64,
            st_blocks: s.st_blocks as i64,
            st_atime: s.st_atime as i64,
            st_mtime: s.st_mtime as i64,
            st_ctime: s.st_ctime as i64,
        }
    }
}

impl ByteRepr for Stat {
    fn byte_size() -> usize {
        144
    }

    fn to_bytes(&self, buf: &mut [u8]) {
        self.st_dev.to_bytes(&mut buf[0..8]);
        self.st_ino.to_bytes(&mut buf[8..16]);
        self.st_nlink.to_bytes(&mut buf[16..24]);
        self.st_mode.to_bytes(&mut buf[24..28]);
        self.st_uid.to_bytes(&mut buf[28..32]);
        self.st_gid.to_bytes(&mut buf[32..36]);
        self.st_rdev.to_bytes(&mut buf[40..48]);
        self.st_size.to_bytes(&mut buf[48..56]);
        self.st_blksize.to_bytes(&mut buf[56..64]);
        self.st_blocks.to_bytes(&mut buf[64..72]);
        self.st_atime.to_bytes(&mut buf[72..80]);
        self.st_mtime.to_bytes(&mut buf[88..96]);
        self.st_ctime.to_bytes(&mut buf[104..112]);
    }

    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            st_dev: u64::from_bytes(&buf[0..8]),
            st_ino: u64::from_bytes(&buf[8..16]),
            st_nlink: u64::from_bytes(&buf[16..24]),
            st_mode: u32::from_bytes(&buf[24..28]),
            st_uid: u32::from_bytes(&buf[28..32]),
            st_gid: u32::from_bytes(&buf[32..36]),
            st_rdev: u64::from_bytes(&buf[40..48]),
            st_size: i64::from_bytes(&buf[48..56]),
            st_blksize: i64::from_bytes(&buf[56..64]),
            st_blocks: i64::from_bytes(&buf[64..72]),
            st_atime: i64::from_bytes(&buf[72..80]),
            st_mtime: i64::from_bytes(&buf[88..96]),
            st_ctime: i64::from_bytes(&buf[104..112]),
        }
    }
}

impl ByteRepr for ::libc::stat {}
