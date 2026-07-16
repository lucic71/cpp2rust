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
            let __de = Dirent::default();
            *__de.d_ino.borrow_mut() = __e.0;
            *__de.d_type.borrow_mut() = __e.2;
            {
                let mut __nm = __de.d_name.borrow_mut();
                let __n = __e.1.len().min(__nm.len() - 1);
                __nm[..__n].copy_from_slice(&__e.1[..__n]);
                __nm[__n] = 0;
            }
            Ptr::alloc(__de)
        }
    })
}

fn f3(a0: Ptr<CDir>) -> i32 {
    a0.delete();
    0
}
