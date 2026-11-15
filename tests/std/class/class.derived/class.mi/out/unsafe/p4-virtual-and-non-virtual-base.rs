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
pub struct FirstVirtualPath {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SecondVirtualPath {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NonVirtualPath {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasSharedAndOwnBase {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: HasSharedAndOwnBase = <HasSharedAndOwnBase>::default();
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut repeated_base_ctors_0)) == (2)));
    let mut firstVirtual: *mut FirstVirtualPath = (&mut d as *mut HasSharedAndOwnBase);
    let mut secondVirtual: *mut SecondVirtualPath = (&mut d as *mut HasSharedAndOwnBase);
    let mut nonVirtual: *mut NonVirtualPath = (&mut d as *mut HasSharedAndOwnBase);
    let mut throughFirstVirtual: *mut RepeatedBase = firstVirtual;
    let mut throughSecondVirtual: *mut RepeatedBase = secondVirtual;
    let mut throughNonVirtual: *mut RepeatedBase = nonVirtual;
    assert!(((throughFirstVirtual) == (throughSecondVirtual)));
    assert!(((throughFirstVirtual) != (throughNonVirtual)));
    ((d as FirstVirtualPath) as RepeatedBase).value = 1;
    ((d as NonVirtualPath) as RepeatedBase).value = 2;
    assert!(((((d as SecondVirtualPath) as RepeatedBase).value) == (1)));
    assert!((((*throughFirstVirtual).value) == (1)));
    assert!((((*throughSecondVirtual).value) == (1)));
    assert!((((*throughNonVirtual).value) == (2)));
    (*throughSecondVirtual).value = 3;
    assert!(((((d as FirstVirtualPath) as RepeatedBase).value) == (3)));
    assert!(((((d as NonVirtualPath) as RepeatedBase).value) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const repeated_base_ctors_0);
}
