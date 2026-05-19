extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn strchr_0(mut s: *const u8, mut c: i32) -> *mut u8 {
    return std::ptr::null_mut();
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: *const u8 = (unsafe {
        let _s: *const u8 = b"hello\0".as_ptr().cast_mut().cast_const();
        let _c: i32 = ('l' as i32);
        strchr_0(_s, _c)
    })
    .cast_const();
    assert!(((((p).is_null()) as i32) != 0));
    return 0;
}
