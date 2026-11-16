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
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct HasTwoBases {
    pub own: i32,
}
impl Default for HasTwoBases {
    fn default() -> Self {
        unsafe { HasTwoBases { own: 3 } }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: HasTwoBases = <HasTwoBases>::default();
    let mut second: *mut SecondBase = (&mut d as *mut HasTwoBases);
    let mut derived: *mut HasTwoBases = (second as *mut HasTwoBases);
    assert!(((derived) == (&mut d as *mut HasTwoBases)));
    assert!((((*(derived as *mut FirstBase)).first) == (1)));
    assert!((((*(derived as *mut SecondBase)).second) == (2)));
    assert!((((*derived).own) == (3)));
    (*(derived as *mut FirstBase)).first = 10;
    assert!((((d as FirstBase).first) == (10)));
    assert!((((*second).second) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
