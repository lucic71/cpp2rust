// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::va_args::VaArgGet;
use crate::{AnyPtr, AsPointer, CFile, Ptr, Value};
use std::cell::{RefCell, UnsafeCell};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;

thread_local! {
    static SAFE_STDIN: Value<std::fs::File> = Rc::new(RefCell::new(std::fs::File::from(
        std::io::stdin().as_fd().try_clone_to_owned().unwrap(),
    )));
    static SAFE_STDOUT: Value<std::fs::File> = Rc::new(RefCell::new(std::fs::File::from(
        std::io::stdout().as_fd().try_clone_to_owned().unwrap(),
    )));
    static SAFE_STDERR: Value<std::fs::File> = Rc::new(RefCell::new(std::fs::File::from(
        std::io::stderr().as_fd().try_clone_to_owned().unwrap(),
    )));
    static UNSAFE_STDIN: UnsafeCell<std::fs::File> = unsafe {
        UnsafeCell::new(
            std::fs::File::from_raw_fd(
                std::io::stdin()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
        ))
    };
    static UNSAFE_STDOUT: UnsafeCell<std::fs::File> = unsafe {
        UnsafeCell::new(
            std::fs::File::from_raw_fd(
                std::io::stdout()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
        ))
    };
    static UNSAFE_STDERR: UnsafeCell<std::fs::File> = unsafe {
        UnsafeCell::new(
            std::fs::File::from_raw_fd(
                std::io::stderr()
                    .as_fd()
                    .try_clone_to_owned()
                    .unwrap()
                    .into_raw_fd(),
        ))
    };
}

thread_local! {
    static C_STDIN: Value<CFile> = Rc::new(RefCell::new(CFile::new(0)));
    static C_STDOUT: Value<CFile> = Rc::new(RefCell::new(CFile::new(1)));
    static C_STDERR: Value<CFile> = Rc::new(RefCell::new(CFile::new(2)));
}

pub fn c_stdin() -> Ptr<CFile> {
    C_STDIN.with(AsPointer::as_pointer)
}

pub fn c_stdout() -> Ptr<CFile> {
    C_STDOUT.with(AsPointer::as_pointer)
}

pub fn c_stderr() -> Ptr<CFile> {
    C_STDERR.with(AsPointer::as_pointer)
}

pub fn cin() -> Ptr<std::fs::File> {
    SAFE_STDIN.with(AsPointer::as_pointer)
}

pub fn cout() -> Ptr<std::fs::File> {
    SAFE_STDOUT.with(AsPointer::as_pointer)
}

pub fn cerr() -> Ptr<std::fs::File> {
    SAFE_STDERR.with(AsPointer::as_pointer)
}

/// # Safety
///
/// The caller must ensure that the returned pointer is not used after the
/// thread finishes.
pub unsafe fn cin_unsafe() -> *mut std::fs::File {
    UNSAFE_STDIN.with(UnsafeCell::get)
}

/// # Safety
///
/// The caller must ensure that the returned pointer is not used after the
/// thread finishes.
pub unsafe fn cout_unsafe() -> *mut std::fs::File {
    UNSAFE_STDOUT.with(UnsafeCell::get)
}

/// # Safety
///
/// The caller must ensure that the returned pointer is not used after the
/// thread finishes.
pub unsafe fn cerr_unsafe() -> *mut std::fs::File {
    UNSAFE_STDERR.with(UnsafeCell::get)
}

pub fn fread_refcount(a0: AnyPtr, a1: usize, a2: usize, a3: Ptr<CFile>) -> usize {
    let total = a1.saturating_mul(a2);
    if total == 0 {
        return 0;
    }
    let dst = a0.reinterpret_cast::<u8>();
    let read_bytes = dst.with_slice_mut(total, |buf| a3.with_mut(|f| f.read(buf)));
    read_bytes / a1
}

pub fn fwrite_refcount(a0: AnyPtr, a1: usize, a2: usize, a3: Ptr<CFile>) -> usize {
    let total = a1.saturating_mul(a2);
    if total == 0 {
        return 0;
    }
    let src = a0.reinterpret_cast::<u8>();
    let written = src.with_slice(total, |bytes| a3.with_mut(|f| f.write(bytes)));
    written / a1
}

unsafe extern "C" {
    #[cfg(target_os = "linux")]
    #[link_name = "stdin"]
    static mut LIBC_STDIN: *mut libc::FILE;
    #[cfg(target_os = "linux")]
    #[link_name = "stdout"]
    static mut LIBC_STDOUT: *mut libc::FILE;
    #[cfg(target_os = "linux")]
    #[link_name = "stderr"]
    static mut LIBC_STDERR: *mut libc::FILE;

    #[cfg(target_os = "macos")]
    #[link_name = "__stdinp"]
    static mut LIBC_STDIN: *mut libc::FILE;
    #[cfg(target_os = "macos")]
    #[link_name = "__stdoutp"]
    static mut LIBC_STDOUT: *mut libc::FILE;
    #[cfg(target_os = "macos")]
    #[link_name = "__stderrp"]
    static mut LIBC_STDERR: *mut libc::FILE;
}

/// # Safety
///
/// Returns the libc `stdin` handle. The pointer is valid for the process
/// lifetime.
pub unsafe fn stdin_unsafe() -> *mut libc::FILE {
    unsafe { LIBC_STDIN }
}

/// # Safety
///
/// Returns the libc `stdout` handle.
pub unsafe fn stdout_unsafe() -> *mut libc::FILE {
    unsafe { LIBC_STDOUT }
}

/// # Safety
///
/// Returns the libc `stderr` handle.
pub unsafe fn stderr_unsafe() -> *mut libc::FILE {
    unsafe { LIBC_STDERR }
}

/// # Safety
///
/// Same contract as C's `fwrite`.
pub unsafe fn fwrite_unsafe(
    a0: *const ::std::ffi::c_void,
    a1: usize,
    a2: usize,
    a3: *mut libc::FILE,
) -> usize {
    unsafe { libc::fwrite(a0, a1, a2, a3) }
}

/// # Safety
///
/// Same contract as C's `fread`.
pub unsafe fn fread_unsafe(
    a0: *mut ::std::ffi::c_void,
    a1: usize,
    a2: usize,
    a3: *mut libc::FILE,
) -> usize {
    unsafe { libc::fread(a0, a1, a2, a3) }
}

/// # Safety
///
/// Same contract as C's `close`.
pub unsafe fn close_unsafe(a0: i32) -> i32 {
    unsafe { libc::close(a0) }
}

/// # Safety
///
/// Same contract as C's `read`.
pub unsafe fn read_unsafe(a0: i32, a1: *mut ::std::ffi::c_void, a2: usize) -> isize {
    unsafe { libc::read(a0, a1, a2) }
}

/// # Safety
///
/// Same contract as C's `write`.
pub unsafe fn write_unsafe(a0: i32, a1: *const ::std::ffi::c_void, a2: usize) -> isize {
    unsafe { libc::write(a0, a1, a2) }
}

/// # Safety
///
/// Same contract as C's `fcntl`, restricted to the three-argument form.
pub unsafe fn fcntl_unsafe(a0: i32, a1: i32, va: &[crate::VaArg]) -> i32 {
    match va.first() {
        None => unsafe { libc::fcntl(a0, a1) },
        Some(crate::VaArg::Int(v)) => unsafe { libc::fcntl(a0, a1, *v) },
        Some(crate::VaArg::UInt(v)) => unsafe { libc::fcntl(a0, a1, *v) },
        Some(crate::VaArg::Long(v)) => unsafe { libc::fcntl(a0, a1, *v) },
        Some(crate::VaArg::ULong(v)) => unsafe { libc::fcntl(a0, a1, *v) },
        Some(crate::VaArg::RawPtr(v)) => unsafe { libc::fcntl(a0, a1, *v) },
        Some(_) => panic!("fcntl_unsafe: unsupported variadic argument"),
    }
}

/// # Safety
///
/// Same contract as C's `ftruncate`.
pub unsafe fn ftruncate_unsafe(a0: i32, a1: libc::off_t) -> i32 {
    unsafe { libc::ftruncate(a0, a1) }
}

/// # Safety
///
/// Same contract as C's `stat`.
pub unsafe fn stat_unsafe(a0: *const libc::c_char, a1: *mut crate::Stat) -> i32 {
    unsafe { libc::stat(a0, a1 as *mut libc::stat) }
}

/// # Safety
///
/// Same contract as C's `fstat`.
pub unsafe fn fstat_unsafe(a0: i32, a1: *mut crate::Stat) -> i32 {
    unsafe { libc::fstat(a0, a1 as *mut libc::stat) }
}

/// # Safety
///
/// Same contract as C's `mkdir`.
pub unsafe fn mkdir_unsafe(a0: *const libc::c_char, a1: libc::mode_t) -> i32 {
    unsafe { libc::mkdir(a0, a1) }
}

/// # Safety
///
/// Same contract as C's `rmdir`.
pub unsafe fn rmdir_unsafe(a0: *const libc::c_char) -> i32 {
    unsafe { libc::rmdir(a0) }
}

/// # Safety
///
/// Same contract as C's `unlink`.
pub unsafe fn unlink_unsafe(a0: *const libc::c_char) -> i32 {
    unsafe { libc::unlink(a0) }
}

/// # Safety
///
/// Same contract as C's `geteuid`.
pub unsafe fn geteuid_unsafe() -> libc::uid_t {
    unsafe { libc::geteuid() }
}

/// # Safety
///
/// Same contract as C's `fclose`.
pub unsafe fn fclose_unsafe(a0: *mut libc::FILE) -> i32 {
    unsafe { libc::fclose(a0) }
}

/// # Safety
///
/// Same contract as C's `pclose`.
pub unsafe fn pclose_unsafe(a0: *mut libc::FILE) -> i32 {
    unsafe { libc::pclose(a0) }
}

/// # Safety
///
/// Same contract as C's `access`.
pub unsafe fn access_unsafe(a0: *const libc::c_char, a1: i32) -> i32 {
    unsafe { libc::access(a0, a1) }
}

/// # Safety
///
/// Same contract as C's `getcwd`.
pub unsafe fn getcwd_unsafe(a0: *mut libc::c_char, a1: usize) -> *mut libc::c_char {
    unsafe { libc::getcwd(a0, a1) }
}

/// # Safety
///
/// Same contract as C's `pread`.
pub unsafe fn pread_unsafe(
    a0: i32,
    a1: *mut ::std::ffi::c_void,
    a2: usize,
    a3: libc::off_t,
) -> isize {
    unsafe { libc::pread(a0, a1, a2, a3) }
}

/// # Safety
///
/// Same contract as C's `pwrite`.
pub unsafe fn pwrite_unsafe(
    a0: i32,
    a1: *const ::std::ffi::c_void,
    a2: usize,
    a3: libc::off_t,
) -> isize {
    unsafe { libc::pwrite(a0, a1, a2, a3) }
}

/// # Safety
///
/// Same contract as C's `fchmod`.
pub unsafe fn fchmod_unsafe(a0: i32, a1: libc::mode_t) -> i32 {
    unsafe { libc::fchmod(a0, a1) }
}

/// # Safety
///
/// Same contract as C's `fchown`.
pub unsafe fn fchown_unsafe(a0: i32, a1: libc::uid_t, a2: libc::gid_t) -> i32 {
    unsafe { libc::fchown(a0, a1, a2) }
}

/// # Safety
///
/// Same contract as C's `readlink`.
pub unsafe fn readlink_unsafe(a0: *const libc::c_char, a1: *mut libc::c_char, a2: usize) -> isize {
    unsafe { libc::readlink(a0, a1, a2) }
}

/// # Safety
///
/// Same contract as C's `lstat`.
pub unsafe fn lstat_unsafe(a0: *const libc::c_char, a1: *mut crate::Stat) -> i32 {
    unsafe { libc::lstat(a0, a1 as *mut libc::stat) }
}

pub fn close_refcount(a0: i32) -> i32 {
    crate::FdRegistry::close(a0)
}

pub fn access_refcount(a0: Ptr<u8>, a1: i32) -> i32 {
    match nix::unistd::access(
        a0.to_rust_string().as_str(),
        nix::unistd::AccessFlags::from_bits_truncate(a1),
    ) {
        Ok(()) => 0,
        Err(e) => {
            crate::cpp2rust_errno().write(e as i32);
            -1
        }
    }
}

pub fn getcwd_refcount(a0: Ptr<u8>, a1: usize) -> Ptr<u8> {
    match nix::unistd::getcwd() {
        Ok(path) => {
            let bytes = path.as_os_str().as_encoded_bytes();
            if bytes.len() + 1 > a1 {
                crate::cpp2rust_errno().write(::libc::ERANGE);
                return Ptr::null();
            }
            a0.with_slice_mut(bytes.len() + 1, |s| {
                s[..bytes.len()].copy_from_slice(bytes);
                s[bytes.len()] = 0;
            });
            a0
        }
        Err(e) => {
            crate::cpp2rust_errno().write(e as i32);
            Ptr::null()
        }
    }
}

pub fn stat_refcount(a0: Ptr<u8>, a1: Ptr<crate::Stat>) -> i32 {
    match nix::sys::stat::stat(a0.to_rust_string().as_str()) {
        Ok(s) => {
            a1.with_mut(|st| *st = crate::Stat::from_libc(&s));
            0
        }
        Err(e) => {
            crate::cpp2rust_errno().write(e as i32);
            -1
        }
    }
}

pub fn lstat_refcount(a0: Ptr<u8>, a1: Ptr<crate::Stat>) -> i32 {
    match nix::sys::stat::lstat(a0.to_rust_string().as_str()) {
        Ok(s) => {
            a1.with_mut(|st| *st = crate::Stat::from_libc(&s));
            0
        }
        Err(e) => {
            crate::cpp2rust_errno().write(e as i32);
            -1
        }
    }
}

pub fn fstat_refcount(a0: i32, a1: Ptr<crate::Stat>) -> i32 {
    match crate::FdRegistry::with_fd(a0, |fd| nix::sys::stat::fstat(fd)) {
        Ok(s) => {
            a1.with_mut(|st| *st = crate::Stat::from_libc(&s));
            0
        }
        Err(e) => {
            crate::cpp2rust_errno().write(e as i32);
            -1
        }
    }
}

pub fn ftruncate_refcount(a0: i32, a1: i64) -> i32 {
    match crate::FdRegistry::with_fd(a0, |fd| nix::unistd::ftruncate(fd, a1)) {
        Ok(()) => 0,
        Err(e) => {
            crate::cpp2rust_errno().write(e as i32);
            -1
        }
    }
}

pub fn fcntl_refcount(a0: i32, a1: i32, va: &[crate::VaArg]) -> i32 {
    if a1 == ::libc::F_SETLK || a1 == ::libc::F_SETLKW || a1 == ::libc::F_GETLK {
        let lk = match va.first() {
            Some(crate::VaArg::Ptr(p)) => p.reinterpret_cast::<crate::Flock>(),
            _ => panic!("fcntl: lock command expects a struct flock pointer"),
        };
        let mut fl = lk.read().to_libc();
        let res = crate::FdRegistry::with_fd(a0, |fd| match a1 {
            ::libc::F_SETLK => nix::fcntl::fcntl(fd, nix::fcntl::FcntlArg::F_SETLK(&fl)),
            ::libc::F_SETLKW => nix::fcntl::fcntl(fd, nix::fcntl::FcntlArg::F_SETLKW(&fl)),
            _ => nix::fcntl::fcntl(fd, nix::fcntl::FcntlArg::F_GETLK(&mut fl)),
        });
        return match res {
            Ok(r) => {
                if a1 == ::libc::F_GETLK {
                    lk.with_mut(|l| *l = crate::Flock::from_libc(&fl));
                }
                r
            }
            Err(e) => {
                crate::cpp2rust_errno().write(e as i32);
                -1
            }
        };
    }
    let res = match a1 {
        ::libc::F_GETFL => crate::FdRegistry::with_fd(a0, |fd| {
            nix::fcntl::fcntl(fd, nix::fcntl::FcntlArg::F_GETFL)
        }),
        ::libc::F_SETFL => {
            let flags = nix::fcntl::OFlag::from_bits_retain(i32::get(&va[0]));
            crate::FdRegistry::with_fd(a0, |fd| {
                nix::fcntl::fcntl(fd, nix::fcntl::FcntlArg::F_SETFL(flags))
            })
        }
        ::libc::F_GETFD => crate::FdRegistry::with_fd(a0, |fd| {
            nix::fcntl::fcntl(fd, nix::fcntl::FcntlArg::F_GETFD)
        }),
        ::libc::F_SETFD => {
            let flags = nix::fcntl::FdFlag::from_bits_retain(i32::get(&va[0]));
            crate::FdRegistry::with_fd(a0, |fd| {
                nix::fcntl::fcntl(fd, nix::fcntl::FcntlArg::F_SETFD(flags))
            })
        }
        cmd => panic!("fcntl: unsupported cmd {}", cmd),
    };
    match res {
        Ok(r) => r,
        Err(e) => {
            crate::cpp2rust_errno().write(e as i32);
            -1
        }
    }
}

pub fn read_refcount(a0: i32, a1: AnyPtr, a2: usize) -> isize {
    match crate::FdRegistry::with_fd(a0, |fd| {
        a1.reinterpret_cast::<u8>()
            .with_slice_mut(a2, |buf| nix::unistd::read(fd, buf))
    }) {
        Ok(n) => n as isize,
        Err(e) => {
            crate::cpp2rust_errno().write(e as i32);
            -1
        }
    }
}

pub fn pread_refcount(a0: i32, a1: AnyPtr, a2: usize, a3: i64) -> isize {
    match crate::FdRegistry::with_fd(a0, |fd| {
        a1.reinterpret_cast::<u8>()
            .with_slice_mut(a2, |buf| nix::sys::uio::pread(fd, buf, a3))
    }) {
        Ok(n) => n as isize,
        Err(e) => {
            crate::cpp2rust_errno().write(e as i32);
            -1
        }
    }
}

pub fn write_refcount(a0: i32, a1: AnyPtr, a2: usize) -> isize {
    match crate::FdRegistry::with_fd(a0, |fd| {
        a1.reinterpret_cast::<u8>()
            .with_slice(a2, |buf| nix::unistd::write(fd, buf))
    }) {
        Ok(n) => n as isize,
        Err(e) => {
            crate::cpp2rust_errno().write(e as i32);
            -1
        }
    }
}

pub fn pwrite_refcount(a0: i32, a1: AnyPtr, a2: usize, a3: i64) -> isize {
    match crate::FdRegistry::with_fd(a0, |fd| {
        a1.reinterpret_cast::<u8>()
            .with_slice(a2, |buf| nix::sys::uio::pwrite(fd, buf, a3))
    }) {
        Ok(n) => n as isize,
        Err(e) => {
            crate::cpp2rust_errno().write(e as i32);
            -1
        }
    }
}

pub fn fchmod_refcount(a0: i32, a1: u32) -> i32 {
    match crate::FdRegistry::with_fd(a0, |fd| {
        nix::sys::stat::fchmod(fd, nix::sys::stat::Mode::from_bits_truncate(a1))
    }) {
        Ok(()) => 0,
        Err(e) => {
            crate::cpp2rust_errno().write(e as i32);
            -1
        }
    }
}

pub fn unlink_refcount(a0: Ptr<u8>) -> i32 {
    match nix::unistd::unlink(a0.to_rust_string().as_str()) {
        Ok(()) => 0,
        Err(e) => {
            crate::cpp2rust_errno().write(e as i32);
            -1
        }
    }
}

pub fn mkdir_refcount(a0: Ptr<u8>, a1: u32) -> i32 {
    match nix::unistd::mkdir(
        a0.to_rust_string().as_str(),
        nix::sys::stat::Mode::from_bits_truncate(a1),
    ) {
        Ok(()) => 0,
        Err(e) => {
            crate::cpp2rust_errno().write(e as i32);
            -1
        }
    }
}

pub fn rmdir_refcount(a0: Ptr<u8>) -> i32 {
    match std::fs::remove_dir(a0.to_rust_string()) {
        Ok(()) => 0,
        Err(e) => {
            crate::cpp2rust_errno().write(e.raw_os_error().unwrap_or(::libc::EIO));
            -1
        }
    }
}

pub fn fchown_refcount(a0: i32, a1: u32, a2: u32) -> i32 {
    match crate::FdRegistry::with_fd(a0, |fd| {
        nix::unistd::fchown(
            fd,
            Some(nix::unistd::Uid::from_raw(a1)),
            Some(nix::unistd::Gid::from_raw(a2)),
        )
    }) {
        Ok(()) => 0,
        Err(e) => {
            crate::cpp2rust_errno().write(e as i32);
            -1
        }
    }
}

pub fn geteuid_refcount() -> u32 {
    nix::unistd::geteuid().as_raw()
}

pub fn readlink_refcount(a0: Ptr<u8>, a1: Ptr<u8>, a2: usize) -> isize {
    match nix::fcntl::readlink(a0.to_rust_string().as_str()) {
        Ok(target) => {
            let bytes = target.as_encoded_bytes();
            let n = bytes.len().min(a2);
            a1.with_slice_mut(n, |s| s[..n].copy_from_slice(&bytes[..n]));
            n as isize
        }
        Err(e) => {
            crate::cpp2rust_errno().write(e as i32);
            -1
        }
    }
}

pub fn fclose_refcount(a0: Ptr<CFile>) -> i32 {
    let r = a0.with(|f| f.close());
    a0.delete();
    r
}

pub fn popen_refcount(_a0: Ptr<u8>, _a1: Ptr<u8>) -> Ptr<CFile> {
    panic!("popen: popen streams are not supported in the refcount model");
}

pub fn pclose_refcount(_a0: Ptr<CFile>) -> i32 {
    panic!("pclose: popen streams are not supported in the refcount model");
}
