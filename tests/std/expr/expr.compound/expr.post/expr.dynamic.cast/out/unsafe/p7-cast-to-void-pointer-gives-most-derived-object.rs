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
    let mut second: *mut SecondBase = (&mut d as *mut MostDerived);
    let mut mostDerived: *mut ::libc::c_void = second;
    let mut addressOfD: *mut ::libc::c_void = ((&mut d as *mut MostDerived) as *mut ::libc::c_void);
    let mut addressOfSecond: *mut ::libc::c_void = (second as *mut ::libc::c_void);
    assert!(((mostDerived) == (addressOfD)));
    assert!(((mostDerived) != (addressOfSecond)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
