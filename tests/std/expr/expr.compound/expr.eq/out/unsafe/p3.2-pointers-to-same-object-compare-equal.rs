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
    let mut derived: *mut HasTwoBases = (&mut d as *mut HasTwoBases);
    let mut first: *mut FirstBase = (&mut d as *mut HasTwoBases);
    let mut second: *mut SecondBase = (&mut d as *mut HasTwoBases);
    assert!(((first) == (derived)));
    assert!(((second) == (derived)));
    assert!(!((second) != (derived)));
    let mut secondAgain: *mut SecondBase = derived;
    assert!(((secondAgain) == (second)));
    let mut nullDerived: *mut HasTwoBases = std::ptr::null_mut();
    let mut nullSecond: *mut SecondBase = nullDerived;
    assert!((nullSecond).is_null());
    assert!(((nullSecond) == (nullDerived)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
