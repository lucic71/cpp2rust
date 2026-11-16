extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Base {
    pub base: i32,
}
impl Default for Base {
    fn default() -> Self {
        unsafe { Base { base: 1 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Derived {
    pub derived: i32,
}
impl Default for Derived {
    fn default() -> Self {
        unsafe { Derived { derived: 2 } }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut onlyBase: Base = <Base>::default();
    let mut base: *mut Base = (&mut onlyBase as *mut Base);
    let mut derived: *mut Derived = (base as *mut Derived);
    return (*derived).derived;
}
pub unsafe fn __cpp2rust_init_globals() {}
