extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut first_calls_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut second_calls_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe trait HasPureVirtual {
    unsafe fn callPure(&mut self) {
        (unsafe { self.pure() });
    }
    unsafe fn pure(&mut self);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct FirstOverrider {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SecondOverrider {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut first: FirstOverrider = <FirstOverrider>::default();
    let mut second: SecondOverrider = <SecondOverrider>::default();
    let mut p: *mut dyn HasPureVirtual = (&mut first as *mut FirstOverrider);
    (unsafe { HasPureVirtual::callPure(&mut (*p)) });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_1)) == (0))
    );
    p = (&mut second as *mut SecondOverrider);
    (unsafe { HasPureVirtual::callPure(&mut (*p)) });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_1)) == (1))
    );
    (unsafe { (*p).pure() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_1)) == (2))
    );
    return 0;
}
unsafe impl HasPureVirtual for FirstOverrider {
    unsafe fn pure(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_0)).prefix_inc();
    }
}
unsafe impl HasPureVirtual for SecondOverrider {
    unsafe fn pure(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_1)).prefix_inc();
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const first_calls_0);
    std::cell::LazyCell::force(&*&raw const second_calls_1);
}
