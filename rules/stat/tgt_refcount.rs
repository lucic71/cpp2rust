// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> libcc2rs::Stat {
    Default::default()
}

fn f1(a0: Ptr<u8>, a1: Ptr<Stat>) -> i32 {
    libcc2rs::stat_refcount(a0.clone(), a1.clone())
}

fn f2(a0: i32, a1: Ptr<Stat>) -> i32 {
    libcc2rs::fstat_refcount(a0, a1.clone())
}

fn f3(a0: Ptr<u8>, a1: ::libc::mode_t) -> i32 {
    libcc2rs::mkdir_refcount(a0.clone(), a1)
}

fn f4(a0: Ptr<u8>, a1: ::libc::mode_t) -> i32 {
    match nix::sys::stat::fchmodat(
        nix::fcntl::AT_FDCWD,
        a0.to_rust_string().as_str(),
        nix::sys::stat::Mode::from_bits_truncate(a1),
        nix::sys::stat::FchmodatFlags::FollowSymlink,
    ) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f5(a0: i32, a1: Ptr<u8>, a2: Ptr<libcc2rs::Timespec>, a3: i32) -> i32 {
    assert!(a0 == ::libc::AT_FDCWD);
    let __at = a2.read();
    let __mt = a2.offset(1).read();
    let __flag = match a3 & ::libc::AT_SYMLINK_NOFOLLOW {
        0 => nix::sys::stat::UtimensatFlags::FollowSymlink,
        _ => nix::sys::stat::UtimensatFlags::NoFollowSymlink,
    };
    match nix::sys::stat::utimensat(
        nix::fcntl::AT_FDCWD,
        a1.to_rust_string().as_str(),
        &nix::sys::time::TimeSpec::new(__at.tv_sec, __at.tv_nsec),
        &nix::sys::time::TimeSpec::new(__mt.tv_sec, __mt.tv_nsec),
        __flag,
    ) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f6(a0: Ptr<u8>, a1: Ptr<Stat>) -> i32 {
    libcc2rs::lstat_refcount(a0.clone(), a1.clone())
}

fn f7(a0: i32, a1: ::libc::mode_t) -> i32 {
    libcc2rs::fchmod_refcount(a0, a1)
}
