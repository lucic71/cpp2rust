extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut second_base_calls_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut derived_calls_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
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
pub struct OverridesSecond {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: OverridesSecond = <OverridesSecond>::default();
    let mut original: *mut SecondBase = (&mut d as *mut OverridesSecond);
    let mut erased: *mut ::libc::c_void = (original as *mut ::libc::c_void);
    let mut restored: *mut SecondBase = (erased as *mut SecondBase);
    assert!(((restored) == (original)));
    assert!((((*restored).second) == (2)));
    (unsafe { (*restored).g() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)) == (1))
    );
    (*restored).second = 5;
    assert!((((d as SecondBase).second) == (5)));
    assert!((((d as FirstBase).first) == (1)));
    return 0;
}
unsafe impl FirstBase for OverridesSecond {
    unsafe fn g(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)).prefix_inc();
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const second_base_calls_0);
    std::cell::LazyCell::force(&*&raw const derived_calls_1);
}
