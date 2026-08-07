// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::ByteRepr;
use crate::libc_shims::time::Timespec;

#[cfg(target_os = "linux")]
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub __pad0: i32,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atim: Timespec,
    pub st_mtim: Timespec,
    pub st_ctim: Timespec,
    pub __glibc_reserved: [i64; 3],
}

#[cfg(target_os = "macos")]
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Stat {
    pub st_dev: i32,
    pub st_mode: u16,
    pub st_nlink: u16,
    pub st_ino: u64,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: i32,
    pub st_atimespec: Timespec,
    pub st_mtimespec: Timespec,
    pub st_ctimespec: Timespec,
    pub st_birthtimespec: Timespec,
    pub st_size: i64,
    pub st_blocks: i64,
    pub st_blksize: i32,
    pub st_flags: u32,
    pub st_gen: u32,
    pub st_lspare: i32,
    pub st_qspare: [i64; 2],
}

const _: () = assert!(size_of::<Stat>() == size_of::<::libc::stat>());
const _: () = assert!(align_of::<Stat>() == align_of::<::libc::stat>());

#[cfg(target_os = "linux")]
const _: () = {
    assert!(core::mem::offset_of!(Stat, st_dev) == 0);
    assert!(core::mem::offset_of!(Stat, st_ino) == 8);
    assert!(core::mem::offset_of!(Stat, st_nlink) == 16);
    assert!(core::mem::offset_of!(Stat, st_mode) == 24);
    assert!(core::mem::offset_of!(Stat, st_uid) == 28);
    assert!(core::mem::offset_of!(Stat, st_gid) == 32);
    assert!(core::mem::offset_of!(Stat, st_rdev) == 40);
    assert!(core::mem::offset_of!(Stat, st_size) == 48);
    assert!(core::mem::offset_of!(Stat, st_blksize) == 56);
    assert!(core::mem::offset_of!(Stat, st_blocks) == 64);
    assert!(core::mem::offset_of!(Stat, st_atim) == 72);
    assert!(core::mem::offset_of!(Stat, st_mtim) == 88);
    assert!(core::mem::offset_of!(Stat, st_ctim) == 104);
};

impl Stat {
    #[cfg(target_os = "linux")]
    #[allow(clippy::unnecessary_cast)]
    pub fn from_libc(s: &::libc::stat) -> Self {
        Self {
            st_dev: s.st_dev as u64,
            st_ino: s.st_ino as u64,
            st_nlink: s.st_nlink as u64,
            st_mode: s.st_mode as u32,
            st_uid: s.st_uid,
            st_gid: s.st_gid,
            __pad0: 0,
            st_rdev: s.st_rdev as u64,
            st_size: s.st_size as i64,
            st_blksize: s.st_blksize as i64,
            st_blocks: s.st_blocks as i64,
            st_atim: Timespec {
                tv_sec: s.st_atime as i64,
                tv_nsec: s.st_atime_nsec as i64,
            },
            st_mtim: Timespec {
                tv_sec: s.st_mtime as i64,
                tv_nsec: s.st_mtime_nsec as i64,
            },
            st_ctim: Timespec {
                tv_sec: s.st_ctime as i64,
                tv_nsec: s.st_ctime_nsec as i64,
            },
            __glibc_reserved: [0; 3],
        }
    }

    #[cfg(target_os = "macos")]
    #[allow(clippy::unnecessary_cast)]
    pub fn from_libc(s: &::libc::stat) -> Self {
        Self {
            st_dev: s.st_dev,
            st_mode: s.st_mode,
            st_nlink: s.st_nlink,
            st_ino: s.st_ino,
            st_uid: s.st_uid,
            st_gid: s.st_gid,
            st_rdev: s.st_rdev,
            st_atimespec: Timespec {
                tv_sec: s.st_atime as i64,
                tv_nsec: s.st_atime_nsec as i64,
            },
            st_mtimespec: Timespec {
                tv_sec: s.st_mtime as i64,
                tv_nsec: s.st_mtime_nsec as i64,
            },
            st_ctimespec: Timespec {
                tv_sec: s.st_ctime as i64,
                tv_nsec: s.st_ctime_nsec as i64,
            },
            st_birthtimespec: Timespec {
                tv_sec: s.st_birthtime as i64,
                tv_nsec: s.st_birthtime_nsec as i64,
            },
            st_size: s.st_size as i64,
            st_blocks: s.st_blocks as i64,
            st_blksize: s.st_blksize as i32,
            st_flags: s.st_flags,
            st_gen: s.st_gen,
            st_lspare: s.st_lspare,
            st_qspare: s.st_qspare,
        }
    }
}

#[cfg(target_os = "linux")]
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
        self.st_atim.to_bytes(&mut buf[72..88]);
        self.st_mtim.to_bytes(&mut buf[88..104]);
        self.st_ctim.to_bytes(&mut buf[104..120]);
    }

    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            st_dev: u64::from_bytes(&buf[0..8]),
            st_ino: u64::from_bytes(&buf[8..16]),
            st_nlink: u64::from_bytes(&buf[16..24]),
            st_mode: u32::from_bytes(&buf[24..28]),
            st_uid: u32::from_bytes(&buf[28..32]),
            st_gid: u32::from_bytes(&buf[32..36]),
            __pad0: 0,
            st_rdev: u64::from_bytes(&buf[40..48]),
            st_size: i64::from_bytes(&buf[48..56]),
            st_blksize: i64::from_bytes(&buf[56..64]),
            st_blocks: i64::from_bytes(&buf[64..72]),
            st_atim: Timespec::from_bytes(&buf[72..88]),
            st_mtim: Timespec::from_bytes(&buf[88..104]),
            st_ctim: Timespec::from_bytes(&buf[104..120]),
            __glibc_reserved: [0; 3],
        }
    }
}

impl ByteRepr for ::libc::stat {}
