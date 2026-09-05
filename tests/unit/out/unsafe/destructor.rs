extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut global_0: i32 = unsafe { 0 };
#[repr(C)]
#[derive(Clone, Default)]
pub struct S {}
impl Drop for S {
    fn drop(&mut self) {
        unsafe {
            global_0.postfix_inc();
        }
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        let mut s: S = S {};
    }
    assert!(((global_0) == (1)));
    {
        let mut s: S = S {};
    }
    assert!(((global_0) == (2)));
    return 0;
}
