extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut total_0: i32 = unsafe { 0 };
#[repr(C)]
#[derive(Clone)]
pub struct Inner {
    pub target: *mut i32,
}
impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            if !((self.target).is_null()) {
                total_0 += (*self.target);
                self.target = std::ptr::null_mut();
            }
        }
    }
}
impl Default for Inner {
    fn default() -> Self {
        Inner {
            target: std::ptr::null_mut(),
        }
    }
}
#[repr(C)]
#[derive(Clone, Default)]
pub struct Outer {
    pub inner: Inner,
}
#[repr(C)]
#[derive(Clone)]
pub struct OutOfLine {
    pub step: i32,
}
impl Drop for OutOfLine {
    fn drop(&mut self) {
        unsafe {
            total_0 += self.step;
        }
    }
}
impl Default for OutOfLine {
    fn default() -> Self {
        OutOfLine { step: 4 }
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut value: i32 = 40;
    {
        let mut o: Outer = <Outer>::default();
        o.inner.target = (&mut value as *mut i32);
    }
    assert!(((total_0) == (40)));
    {
        let mut t: OutOfLine = <OutOfLine>::default();
    }
    assert!(((total_0) == (44)));
    return 0;
}
