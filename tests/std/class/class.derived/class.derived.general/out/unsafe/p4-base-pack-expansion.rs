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
pub struct CountsFirst {
    pub first: i32,
}
impl CountsFirst {
    pub unsafe fn touchFirst(&mut self) {
        self.first.prefix_inc();
    }
}
impl Default for CountsFirst {
    fn default() -> Self {
        unsafe { CountsFirst { first: 0 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct CountsSecond {
    pub second: i32,
}
impl CountsSecond {
    pub unsafe fn touchSecond(&mut self) {
        self.second.prefix_inc();
    }
}
impl Default for CountsSecond {
    fn default() -> Self {
        unsafe { CountsSecond { second: 0 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct DerivesFromPack_CountsFirst__CountsSecond_ {
    pub own: i32,
}
impl Default for DerivesFromPack_CountsFirst__CountsSecond_ {
    fn default() -> Self {
        unsafe { DerivesFromPack_CountsFirst__CountsSecond_ { own: 0 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct DerivesFromPack_CountsFirst_ {
    pub own: i32,
}
impl Default for DerivesFromPack_CountsFirst_ {
    fn default() -> Self {
        unsafe { DerivesFromPack_CountsFirst_ { own: 0 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct DerivesFromPack__ {
    pub own: i32,
}
impl Default for DerivesFromPack__ {
    fn default() -> Self {
        unsafe { DerivesFromPack__ { own: 0 } }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut both: DerivesFromPack_CountsFirst__CountsSecond_ =
        <DerivesFromPack_CountsFirst__CountsSecond_>::default();
    (unsafe { CountsFirst::touchFirst(&mut (both as CountsFirst)) });
    assert!((((both as CountsFirst).first) == (1)) && (((both as CountsSecond).second) == (0)));
    (unsafe { CountsSecond::touchSecond(&mut (both as CountsSecond)) });
    assert!((((both as CountsFirst).first) == (1)) && (((both as CountsSecond).second) == (1)));
    let mut second: *mut CountsSecond =
        (&mut both as *mut DerivesFromPack_CountsFirst__CountsSecond_);
    (unsafe { CountsSecond::touchSecond(&mut (*second)) });
    assert!((((both as CountsFirst).first) == (1)) && (((both as CountsSecond).second) == (2)));
    let mut one: DerivesFromPack_CountsFirst_ = <DerivesFromPack_CountsFirst_>::default();
    (unsafe { CountsFirst::touchFirst(&mut (one as CountsFirst)) });
    assert!((((one as CountsFirst).first) == (1)));
    assert!((((both as CountsFirst).first) == (1)));
    let mut none: DerivesFromPack__ = <DerivesFromPack__>::default();
    none.own = 1;
    assert!(((none.own) == (1)));
    assert!(((both.own) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
