extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Empty {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct FirstPath {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SecondPath {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasTwoEmptyBases {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: HasTwoEmptyBases = <HasTwoEmptyBases>::default();
    let mut first: *mut FirstPath = (&mut d as *mut HasTwoEmptyBases);
    let mut second: *mut SecondPath = (&mut d as *mut HasTwoEmptyBases);
    let mut throughFirst: *mut Empty = first;
    let mut throughSecond: *mut Empty = second;
    assert!(((throughFirst) != (throughSecond)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
