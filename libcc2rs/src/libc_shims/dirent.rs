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
        let mut de = Dirent::default();
        de.d_ino = ino;
        de.d_type = d_type;
        {
            let nm = &mut de.d_name;
            let n = name.len().min(nm.len() - 1);
            nm[..n].copy_from_slice(&name[..n]);
            nm[n] = 0;
        }
        de
    }
}

impl ByteRepr for Dirent {}

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

impl ByteRepr for CDir {}

impl ByteRepr for ::libc::dirent {}
