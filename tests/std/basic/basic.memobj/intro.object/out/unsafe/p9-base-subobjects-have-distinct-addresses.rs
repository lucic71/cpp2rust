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
pub struct FirstBase {
    pub first: i32,
}
impl Default for FirstBase {
    fn default() -> Self {
        unsafe { FirstBase { first: 1 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct SecondBase {
    pub second: i32,
}
impl Default for SecondBase {
    fn default() -> Self {
        unsafe { SecondBase { second: 2 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasTwoBases {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: HasTwoBases = <HasTwoBases>::default();
    let mut first: *mut FirstBase = (&mut d as *mut HasTwoBases);
    let mut second: *mut SecondBase = (&mut d as *mut HasTwoBases);
    let mut firstAddress: *mut ::libc::c_void = (first as *mut ::libc::c_void);
    let mut secondAddress: *mut ::libc::c_void = (second as *mut ::libc::c_void);
    assert!(((firstAddress) != (secondAddress)));
    (*first).first = 10;
    assert!((((d as FirstBase).first) == (10)) && (((d as SecondBase).second) == (2)));
    (*second).second = 20;
    assert!((((d as FirstBase).first) == (10)) && (((d as SecondBase).second) == (20)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
