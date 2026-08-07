// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1(a0: i32) -> i32 {
    libcc2rs::close_refcount(a0)
}

fn f2(a0: i32, a1: i64, a2: i32) -> i64 {
    let __whence = match a2 {
        0 => nix::unistd::Whence::SeekSet,
        1 => nix::unistd::Whence::SeekCur,
        2 => nix::unistd::Whence::SeekEnd,
        __w => panic!("lseek: unsupported whence {__w}"),
    };
    match FdRegistry::with_fd(a0, |__fd| nix::unistd::lseek(__fd, a1, __whence)) {
        Ok(__off) => __off,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f3(a0: i32, a1: AnyPtr, a2: usize) -> isize {
    libcc2rs::read_refcount(a0, a1.clone(), a2)
}

fn f4(a0: Ptr<u8>) -> i32 {
    libcc2rs::unlink_refcount(a0.clone())
}

fn f5(a0: Ptr<i32>) -> i32 {
    match nix::unistd::pipe() {
        Ok((__r, __w)) => {
            let __fds = a0.clone();
            __fds.write(FdRegistry::register(__r));
            __fds.offset(1).write(FdRegistry::register(__w));
            0
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f6(a0: i32, a1: i64) -> i32 {
    libcc2rs::ftruncate_refcount(a0, a1)
}

fn f7(a0: i32) -> i32 {
    match FdRegistry::with_fd(a0, |__fd| nix::unistd::isatty(__fd)) {
        Ok(__tty) => __tty as i32,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            0
        }
    }
}

fn f8() -> u32 {
    libcc2rs::geteuid_refcount()
}

fn f9(a0: Ptr<u8>, a1: usize) -> i32 {
    match nix::unistd::gethostname() {
        Ok(__name) => {
            let __bytes = __name.as_encoded_bytes();
            let __n = __bytes.len().min(a1.saturating_sub(1));
            if a1 > 0 {
                a0.with_slice_mut(__n + 1, |__s| {
                    __s[..__n].copy_from_slice(&__bytes[..__n]);
                    __s[__n] = 0;
                });
            }
            0
        }
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f10(a0: i32, a1: AnyPtr, a2: usize) -> isize {
    libcc2rs::write_refcount(a0, a1.clone(), a2)
}

fn f11(a0: Ptr<u8>) -> i32 {
    libcc2rs::rmdir_refcount(a0.clone())
}

fn f12(a0: Ptr<u8>, a1: ::libc::uid_t, a2: ::libc::gid_t) -> i32 {
    match nix::unistd::chown(
        a0.to_rust_string().as_str(),
        Some(nix::unistd::Uid::from_raw(a1)),
        Some(nix::unistd::Gid::from_raw(a2)),
    ) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f18(a0: i32) -> i32 {
    match FdRegistry::with_fd(a0, |__fd| nix::unistd::fsync(__fd)) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f21() -> i32 {
    nix::unistd::getpid().as_raw()
}

fn f22() -> u32 {
    nix::unistd::getuid().as_raw()
}

fn f13(a0: Ptr<u8>, a1: i32) -> i32 {
    libcc2rs::access_refcount(a0.clone(), a1)
}

fn f14(a0: Ptr<u8>, a1: Ptr<u8>, a2: usize) -> isize {
    libcc2rs::readlink_refcount(a0.clone(), a1.clone(), a2)
}

fn f16(a0: Ptr<u8>, a1: usize) -> Ptr<u8> {
    libcc2rs::getcwd_refcount(a0.clone(), a1)
}

fn f19(a0: i32, a1: AnyPtr, a2: usize, a3: i64) -> isize {
    libcc2rs::pread_refcount(a0, a1.clone(), a2, a3)
}

fn f20(a0: i32, a1: AnyPtr, a2: usize, a3: i64) -> isize {
    libcc2rs::pwrite_refcount(a0, a1.clone(), a2, a3)
}

fn f23(a0: i32, a1: ::libc::uid_t, a2: ::libc::gid_t) -> i32 {
    libcc2rs::fchown_refcount(a0, a1, a2)
}

fn f15(a0: Ptr<u8>, a1: Ptr<u8>) -> i32 {
    match ::std::os::unix::fs::symlink(a0.to_rust_string(), a1.to_rust_string()) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e.raw_os_error().unwrap_or(::libc::EIO));
            -1
        }
    }
}

fn f17(a0: Ptr<u8>) -> i32 {
    match nix::unistd::chdir(a0.to_rust_string().as_str()) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f24(a0: i32) {
    ::std::process::exit(a0)
}

fn f25(a0: Ptr<u8>, a1: Ptr<Ptr<u8>>, a2: Ptr<Ptr<u8>>) -> i32 {
    panic!("execve: process replacement is not supported in the refcount model")
}

fn f26() -> i32 {
    panic!("fork: child processes are not supported in the refcount model")
}

fn f27(a0: i32) -> i32 {
    match FdRegistry::with_fd(a0, |__fd| nix::unistd::dup(__fd)) {
        Ok(__new) => FdRegistry::register(__new),
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f28(a0: i32, a1: i32) -> i32 {
    panic!("dup2: fd renumbering is not supported in the refcount model")
}

fn f29(a0: i32) -> i64 {
    match nix::unistd::sysconf(match a0 {
        ::libc::_SC_OPEN_MAX => nix::unistd::SysconfVar::OPEN_MAX,
        ::libc::_SC_PAGESIZE => nix::unistd::SysconfVar::PAGE_SIZE,
        _ => panic!("sysconf: unsupported name"),
    }) {
        Ok(Some(__v)) => __v,
        Ok(None) => -1,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f30() -> i32 {
    ::libc::_SC_OPEN_MAX
}

fn f31(a0: u32) -> i32 {
    match nix::unistd::setuid(nix::unistd::Uid::from_raw(a0)) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f32(a0: u32) -> i32 {
    match nix::unistd::setgid(nix::unistd::Gid::from_raw(a0)) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f33(a0: usize, a1: Ptr<u32>) -> i32 {
    let mut __gids = Vec::with_capacity(a0);
    let mut __i = 0;
    while __i < a0 {
        __gids.push(nix::unistd::Gid::from_raw(a1.clone().offset(__i).read()));
        __i += 1;
    }
    match nix::unistd::setgroups(&__gids) {
        Ok(()) => 0,
        Err(__e) => {
            libcc2rs::cpp2rust_errno().write(__e as i32);
            -1
        }
    }
}

fn f34() -> Ptr<Ptr<u8>> {
    libcc2rs::cpp2rust_environ()
}
