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
pub struct MostDerived {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: MostDerived = <MostDerived>::default();
    let mut first: *mut FirstBase = (&mut d as *mut MostDerived);
    let mut second: *mut SecondBase = first;
    let mut expected: *mut SecondBase = (&mut d as *mut MostDerived);
    assert!(((second) == (expected)));
    assert!((((*second).second) == (2)));
    (*second).second = 5;
    assert!((((d as SecondBase).second) == (5)));
    assert!((((d as FirstBase).first) == (1)));
    let mut back: *mut FirstBase = second;
    assert!(((back) == (first)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
