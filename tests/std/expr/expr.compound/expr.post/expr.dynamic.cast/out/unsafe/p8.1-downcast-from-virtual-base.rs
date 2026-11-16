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
pub struct VirtualBase {
    pub shared: i32,
}
impl Default for VirtualBase {
    fn default() -> Self {
        unsafe { VirtualBase { shared: 1 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct FirstPath {
    pub first: i32,
}
impl Default for FirstPath {
    fn default() -> Self {
        unsafe { FirstPath { first: 2 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct SecondPath {
    pub second: i32,
}
impl Default for SecondPath {
    fn default() -> Self {
        unsafe { SecondPath { second: 3 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct JoinsPaths {
    pub own: i32,
}
impl Default for JoinsPaths {
    fn default() -> Self {
        unsafe { JoinsPaths { own: 4 } }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: JoinsPaths = <JoinsPaths>::default();
    let mut base: *mut VirtualBase = (&mut d as *mut JoinsPaths);
    let mut toJoin: *mut JoinsPaths = base;
    assert!(((toJoin) == (&mut d as *mut JoinsPaths)));
    assert!((((*toJoin).own) == (4)));
    let mut toSecond: *mut SecondPath = base;
    let mut expected: *mut SecondPath = (&mut d as *mut JoinsPaths);
    assert!(((toSecond) == (expected)));
    assert!((((*toSecond).second) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
