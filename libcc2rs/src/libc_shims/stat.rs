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

impl ByteRepr for Stat {}

impl ByteRepr for ::libc::stat {}
