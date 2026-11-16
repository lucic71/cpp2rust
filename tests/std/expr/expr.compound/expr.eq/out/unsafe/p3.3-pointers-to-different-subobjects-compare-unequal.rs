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
    let mut a: HasTwoBases = <HasTwoBases>::default();
    let mut b: HasTwoBases = <HasTwoBases>::default();
    let mut secondOfA: *mut SecondBase = (&mut a as *mut HasTwoBases);
    let mut secondOfB: *mut SecondBase = (&mut b as *mut HasTwoBases);
    assert!(((secondOfA) != (secondOfB)));
    assert!(!((secondOfA) == (secondOfB)));
    let mut firstOfA: *mut FirstBase = (&mut a as *mut HasTwoBases);
    let mut firstAddress: *mut ::libc::c_void = (firstOfA as *mut ::libc::c_void);
    let mut secondAddress: *mut ::libc::c_void = (secondOfA as *mut ::libc::c_void);
    assert!(((firstAddress) != (secondAddress)));
    let mut nullSecond: *mut SecondBase = std::ptr::null_mut();
    assert!(((secondOfA) != (nullSecond)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
