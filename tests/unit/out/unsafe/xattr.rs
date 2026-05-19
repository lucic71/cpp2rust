extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_fsetxattr_bad_fd_0() {
    assert!(
        ((((libc::fsetxattr(
            -1_i32,
            (b"user.test\0".as_ptr().cast_mut()).cast_const() as *const i8,
            (b"v\0".as_ptr().cast_mut() as *const u8 as *const ::libc::c_void),
            1_u64 as usize,
            0
        )) == (-1_i32)) as i32)
            != 0)
    );
}
pub unsafe fn test_fsetxattr_roundtrip_1() {
    let mut path: *const u8 = (b"/tmp/cpp2rust_xattr_test.tmp\0".as_ptr().cast_mut()).cast_const();
    let mut fp: *mut ::std::fs::File =
        match std::ffi::CStr::from_ptr((b"wb\0".as_ptr().cast_mut()).cast_const() as *const i8)
            .to_str()
            .expect("invalid c-string")
        {
            v if v == "rb" => std::fs::OpenOptions::new()
                .read(true)
                .open(
                    std::ffi::CStr::from_ptr(path as *const i8)
                        .to_str()
                        .expect("invalid c-string"),
                )
                .ok()
                .map_or(std::ptr::null_mut(), |f| Box::into_raw(Box::new(f))),
            v if v == "wb" => std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(
                    std::ffi::CStr::from_ptr(path as *const i8)
                        .to_str()
                        .expect("invalid c-string"),
                )
                .ok()
                .map_or(std::ptr::null_mut(), |f| Box::into_raw(Box::new(f))),
            _ => panic!("unsupported mode"),
        };
    assert!((((!((fp).is_null())) as i32) != 0));
    let mut fd: i32 = if fp == libcc2rs::cin_unsafe() {
        0
    } else if fp == libcc2rs::cout_unsafe() {
        1
    } else if fp == libcc2rs::cerr_unsafe() {
        2
    } else {
        ::std::os::fd::AsRawFd::as_raw_fd(&*fp)
    };
    assert!(((((fd) != (-1_i32)) as i32) != 0));
    assert!(
        ((((libc::fsetxattr(
            fd,
            (b"user.test\0".as_ptr().cast_mut()).cast_const() as *const i8,
            (b"value\0".as_ptr().cast_mut() as *const u8 as *const ::libc::c_void),
            5_u64 as usize,
            0
        )) == (0)) as i32)
            != 0)
    );
    let mut buf: [u8; 16] = [0_u8; 16];
    assert!(
        ((((libc::fgetxattr(
            fd,
            (b"user.test\0".as_ptr().cast_mut()).cast_const() as *const i8,
            (buf.as_mut_ptr() as *mut u8 as *mut ::libc::c_void),
            ::std::mem::size_of::<[u8; 16]>() as u64 as usize
        ) as i64)
            == (5_i64)) as i32)
            != 0)
    );
    assert!(
        (((({
            let sa = core::slice::from_raw_parts(
                (buf.as_mut_ptr() as *const u8 as *const ::libc::c_void) as *const u8,
                5_u64 as usize,
            );
            let sb = core::slice::from_raw_parts(
                (b"value\0".as_ptr().cast_mut() as *const u8 as *const ::libc::c_void) as *const u8,
                5_u64 as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0)) as i32)
            != 0)
    );
    assert!(
        (((({
            Box::from_raw(fp);
            0
        }) == (0)) as i32)
            != 0)
    );
    assert!(((((libc::unlink(path as *const i8)) == (0)) as i32) != 0));
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_fsetxattr_bad_fd_0() });
    (unsafe { test_fsetxattr_roundtrip_1() });
    return 0;
}
