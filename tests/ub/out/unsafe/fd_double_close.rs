extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut fd: i32 = (unsafe {
        libc::open(
            (c"/dev/null".as_ptr().cast_mut()).cast_const() as *const i8,
            ::libc::O_RDONLY as i32,
        )
    });
    libcc2rs::close_unsafe(fd);
    return if (((libcc2rs::close_unsafe(fd)) == (-1_i32)) as i32) != 0 {
        0
    } else {
        1
    };
}
