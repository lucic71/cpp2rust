extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut repeated_base_ctors_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct RepeatedBase {
    pub value: i32,
}
impl RepeatedBase {
    pub unsafe fn new() -> Self {
        let mut this = Self { value: 0 };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut repeated_base_ctors_0)).prefix_inc();
        this
    }
}
impl Default for RepeatedBase {
    fn default() -> Self {
        unsafe { RepeatedBase::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct FirstPath {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SecondPath {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasTwoRepeatedBases {}
impl HasTwoRepeatedBases {
    pub unsafe fn copyValueFromSecondToFirst(&mut self) {
        self.value = self.value;
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: HasTwoRepeatedBases = <HasTwoRepeatedBases>::default();
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut repeated_base_ctors_0)) == (2)));
    ((d as FirstPath) as RepeatedBase).value = 1;
    ((d as SecondPath) as RepeatedBase).value = 2;
    assert!(((((d as FirstPath) as RepeatedBase).value) == (1)));
    assert!(((((d as SecondPath) as RepeatedBase).value) == (2)));
    let mut firstPath: *mut FirstPath = (&mut d as *mut HasTwoRepeatedBases);
    let mut secondPath: *mut SecondPath = (&mut d as *mut HasTwoRepeatedBases);
    let mut baseOfFirst: *mut RepeatedBase = firstPath;
    let mut baseOfSecond: *mut RepeatedBase = secondPath;
    assert!(((baseOfFirst) != (baseOfSecond)));
    assert!((((*baseOfFirst).value) == (1)));
    assert!((((*baseOfSecond).value) == (2)));
    (unsafe { HasTwoRepeatedBases::copyValueFromSecondToFirst(&mut d) });
    assert!((((*baseOfFirst).value) == (2)));
    assert!((((*baseOfSecond).value) == (2)));
    (*baseOfSecond).value = 3;
    assert!(((((d as FirstPath) as RepeatedBase).value) == (2)));
    assert!(((((d as SecondPath) as RepeatedBase).value) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const repeated_base_ctors_0);
}
