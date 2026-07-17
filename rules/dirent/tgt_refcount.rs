// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> Ptr<libcc2rs::CDir> {
    Ptr::null()
}

fn t2() -> libcc2rs::Dirent {
    Default::default()
}

fn f1(a0: Ptr<u8>) -> Ptr<CDir> {
    match nix::dir::Dir::open(
        a0.to_rust_string().as_str(),
        nix::fcntl::OFlag::O_RDONLY,
        nix::sys::stat::Mode::empty(),
    ) {
        Ok(__dir) => {
            let mut __entries: Vec<(u64, Vec<u8>, u8)> = Vec::new();
            for __e in __dir {
                match __e {
                    Ok(__ent) => {
                        let __ty = match __ent.file_type() {
                            Some(nix::dir::Type::Fifo) => ::libc::DT_FIFO,
                            Some(nix::dir::Type::CharacterDevice) => ::libc::DT_CHR,
                            Some(nix::dir::Type::Directory) => ::libc::DT_DIR,
                            Some(nix::dir::Type::BlockDevice) => ::libc::DT_BLK,
                            Some(nix::dir::Type::File) => ::libc::DT_REG,
                            Some(nix::dir::Type::Symlink) => ::libc::DT_LNK,
                            Some(nix::dir::Type::Socket) => ::libc::DT_SOCK,
                            None => ::libc::DT_UNKNOWN,
                        };
                        __entries.push((__ent.ino(), __ent.file_name().to_bytes().to_vec(), __ty));
                    }
                    Err(_) => {}
                }
            }
            Ptr::alloc(CDir {
                entries: __entries,
                pos: ::std::cell::Cell::new(0),
            })
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            Ptr::null()
        }
    }
}

fn f2(a0: Ptr<CDir>) -> Ptr<Dirent> {
    a0.with(|__d| {
        let __i = __d.pos.get();
        if __i >= __d.entries.len() {
            Ptr::null()
        } else {
            __d.pos.set(__i + 1);
            let __e = &__d.entries[__i];
            Ptr::alloc(Dirent::from_entry(__e.0, &__e.1, __e.2))
        }
    })
}

fn f3(a0: Ptr<CDir>) -> i32 {
    a0.delete();
    0
}
