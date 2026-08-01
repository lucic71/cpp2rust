// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::ByteRepr;
use std::cell::Cell;

#[derive(Clone)]
pub struct Dirent {
    pub d_ino: u64,
    pub d_off: i64,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: Box<[u8]>,
}

impl Default for Dirent {
    fn default() -> Self {
        Self {
            d_ino: 0,
            d_off: 0,
            d_reclen: 0,
            d_type: 0,
            d_name: vec![0u8; 256].into_boxed_slice(),
        }
    }
}

impl Dirent {
    pub fn from_entry(ino: u64, name: &[u8], d_type: u8) -> Self {
        let mut de = Dirent {
            d_ino: ino,
            d_type,
            ..Default::default()
        };
        {
            let nm = &mut de.d_name;
            let n = name.len().min(nm.len() - 1);
            nm[..n].copy_from_slice(&name[..n]);
            nm[n] = 0;
        }
        de
    }
}

impl ByteRepr for Dirent {
    fn byte_size() -> usize {
        280
    }

    fn to_bytes(&self, buf: &mut [u8]) {
        self.d_ino.to_bytes(&mut buf[0..8]);
        self.d_off.to_bytes(&mut buf[8..16]);
        self.d_reclen.to_bytes(&mut buf[16..18]);
        self.d_type.to_bytes(&mut buf[18..19]);
        buf[19..275].copy_from_slice(&self.d_name);
    }

    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            d_ino: u64::from_bytes(&buf[0..8]),
            d_off: i64::from_bytes(&buf[8..16]),
            d_reclen: u16::from_bytes(&buf[16..18]),
            d_type: u8::from_bytes(&buf[18..19]),
            d_name: buf[19..275].to_vec().into_boxed_slice(),
        }
    }
}

pub struct CDir {
    pub entries: Vec<(u64, Vec<u8>, u8)>,
    pub pos: Cell<usize>,
}

impl CDir {
    pub fn from_dir(dir: nix::dir::Dir) -> Self {
        let mut entries: Vec<(u64, Vec<u8>, u8)> = Vec::new();
        for ent in dir.into_iter().flatten() {
            let ty = match ent.file_type() {
                Some(nix::dir::Type::Fifo) => ::libc::DT_FIFO,
                Some(nix::dir::Type::CharacterDevice) => ::libc::DT_CHR,
                Some(nix::dir::Type::Directory) => ::libc::DT_DIR,
                Some(nix::dir::Type::BlockDevice) => ::libc::DT_BLK,
                Some(nix::dir::Type::File) => ::libc::DT_REG,
                Some(nix::dir::Type::Symlink) => ::libc::DT_LNK,
                Some(nix::dir::Type::Socket) => ::libc::DT_SOCK,
                None => ::libc::DT_UNKNOWN,
            };
            entries.push((ent.ino(), ent.file_name().to_bytes().to_vec(), ty));
        }
        Self {
            entries,
            pos: Cell::new(0),
        }
    }
}

impl ByteRepr for CDir {
    fn byte_size() -> usize {
        0
    }
}

impl ByteRepr for ::libc::dirent {}
