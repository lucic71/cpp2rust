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
pub static mut final_calls_3: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DeclaresVirtual {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct FirstVirtualOverrider {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SecondVirtualOverrider {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct OverridesBoth {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: OverridesBoth = <OverridesBoth>::default();
    let mut base: *mut DeclaresVirtual = (&mut d as *mut OverridesBoth);
    let mut first: *mut FirstVirtualOverrider = (&mut d as *mut OverridesBoth);
    let mut second: *mut SecondVirtualOverrider = (&mut d as *mut OverridesBoth);
    (unsafe { (*base).f() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut final_calls_3)) == (1)));
    (unsafe { (*first).f() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut final_calls_3)) == (2)));
    (unsafe { (*second).f() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut final_calls_3)) == (3)));
    (unsafe { d.f() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut final_calls_3)) == (4)));
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_1)) == (0)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_2)) == (0))
    );
    (unsafe { (d as FirstVirtualOverrider).f() });
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_1)) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_2)) == (0))
    );
    (unsafe { (d as SecondVirtualOverrider).f() });
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_1)) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_2)) == (1))
    );
    (unsafe { (d as DeclaresVirtual).f() });
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_1)) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_2)) == (1))
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut final_calls_3)) == (4)));
    return 0;
}
unsafe impl DeclaresVirtual for FirstVirtualOverrider {
    unsafe fn f(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_1)).prefix_inc();
    }
}
unsafe impl FirstVirtualOverrider for OverridesBoth {
    unsafe fn f(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut final_calls_3)).prefix_inc();
    }
}
unsafe impl DeclaresVirtual for SecondVirtualOverrider {
    unsafe fn f(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_2)).prefix_inc();
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const base_calls_0);
    std::cell::LazyCell::force(&*&raw const first_calls_1);
    std::cell::LazyCell::force(&*&raw const second_calls_2);
    std::cell::LazyCell::force(&*&raw const final_calls_3);
}
