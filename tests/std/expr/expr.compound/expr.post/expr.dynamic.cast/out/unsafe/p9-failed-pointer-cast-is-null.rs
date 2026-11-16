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
pub struct Base {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct FirstDerived {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SecondDerived {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut onlyBase: Base = <Base>::default();
    let mut base: *mut Base = (&mut onlyBase as *mut Base);
    assert!((base).is_null());
    let mut second: SecondDerived = <SecondDerived>::default();
    base = (&mut second as *mut SecondDerived);
    assert!((base).is_null());
    assert!(((base) == (&mut second as *mut SecondDerived)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
