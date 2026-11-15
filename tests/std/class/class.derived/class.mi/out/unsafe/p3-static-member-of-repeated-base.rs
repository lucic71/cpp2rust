extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut ctorCount_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut touchCount_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub type RepeatedBase_Kind = u32;
pub const RepeatedBase_Kind_Plain: RepeatedBase_Kind = 7;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct RepeatedBase {
    pub value: i32,
}
impl RepeatedBase {
    pub unsafe fn new() -> Self {
        let mut this = Self { value: 0 };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)).prefix_inc();
        this
    }
    pub unsafe fn touch() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut touchCount_1)).prefix_inc();
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
    pub unsafe fn touchFromMember(&mut self) {
        (unsafe { RepeatedBase::touch() });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DirectAndIndirect {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: HasTwoRepeatedBases = <HasTwoRepeatedBases>::default();
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (2)));
    (unsafe { RepeatedBase::touch() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut touchCount_1)) == (1)));
    (unsafe { HasTwoRepeatedBases::touchFromMember(&mut c) });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut touchCount_1)) == (2)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut touchCount_1)) == (2)));
    assert!(((RepeatedBase_Kind_Plain as i32) == (7)));
    let mut d: DirectAndIndirect = <DirectAndIndirect>::default();
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut ctorCount_0)) == (4)));
    (unsafe { RepeatedBase::touch() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut touchCount_1)) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const ctorCount_0);
    std::cell::LazyCell::force(&*&raw const touchCount_1);
}
