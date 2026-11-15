extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut base_calls_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut first_calls_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut second_calls_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DeclaresVirtual {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct FirstOverrider {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SecondOverrider {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasTwoBaseSubobjects {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: HasTwoBaseSubobjects = <HasTwoBaseSubobjects>::default();
    let mut first: *mut FirstOverrider = (&mut d as *mut HasTwoBaseSubobjects);
    let mut base: *mut DeclaresVirtual = first;
    (unsafe { (*base).f() });
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_1)) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_2)) == (0))
    );
    let mut second: *mut SecondOverrider = (&mut d as *mut HasTwoBaseSubobjects);
    base = second;
    (unsafe { (*base).f() });
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_1)) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_2)) == (1))
    );
    (unsafe { (*first).f() });
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_1)) == (2)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_2)) == (1))
    );
    (unsafe { (*second).f() });
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_1)) == (2)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_2)) == (2))
    );
    return 0;
}
unsafe impl DeclaresVirtual for FirstOverrider {
    unsafe fn f(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_1)).prefix_inc();
    }
}
unsafe impl DeclaresVirtual for SecondOverrider {
    unsafe fn f(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_2)).prefix_inc();
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const base_calls_0);
    std::cell::LazyCell::force(&*&raw const first_calls_1);
    std::cell::LazyCell::force(&*&raw const second_calls_2);
}
