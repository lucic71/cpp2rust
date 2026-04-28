extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(mut str: *mut u8) {}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut mutable_strings: [*mut u8; 3] = [
        b"a\0".as_ptr().cast_mut(),
        b"b\0".as_ptr().cast_mut(),
        b"c\0".as_ptr().cast_mut(),
    ];
    let mut immutable_strings: [*const u8; 3] = [
        b"a\0".as_ptr().cast_mut().cast_const(),
        b"b\0".as_ptr().cast_mut().cast_const(),
        b"c\0".as_ptr().cast_mut().cast_const(),
    ];
    let mut mutable_string: *mut u8 = b"hello\0".as_ptr().cast_mut();
    let mut immutable_string: *const u8 = b"hello\0".as_ptr().cast_mut().cast_const();
    (unsafe {
        let _str: *mut u8 = b"world\0".as_ptr().cast_mut();
        foo_0(_str)
    });
    (unsafe {
        let _str: *mut u8 = mutable_string;
        foo_0(_str)
    });
    return 0;
}
