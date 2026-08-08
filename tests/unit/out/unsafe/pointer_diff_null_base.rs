extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut buf_0: [u8; 16] = unsafe { [0_u8; 16] };
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: *const u8 = (buf_0.as_mut_ptr()).cast_const();
    return (((((((p as usize) - ((0 as *const u8) as usize)) / ::std::mem::size_of::<u8>())
        as i64)
        & (7_i64))
        == (0_i64)) as i32);
}
