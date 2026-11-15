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
pub static mut final_calls_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DeclaresVirtual {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct OverridesAsFinal {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DerivesFromFinalOverrider {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct FinalClass {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: DerivesFromFinalOverrider = <DerivesFromFinalOverrider>::default();
    let mut p: *mut DeclaresVirtual = (&mut d as *mut DerivesFromFinalOverrider);
    (unsafe { (*p).f() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut final_calls_1)) == (1))
    );
    let mut q: *mut OverridesAsFinal = (&mut d as *mut DerivesFromFinalOverrider);
    (unsafe { (*q).f() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut final_calls_1)) == (2))
    );
    let mut c: FinalClass = <FinalClass>::default();
    p = (&mut c as *mut FinalClass);
    (unsafe { (*p).f() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut final_calls_1)) == (3))
    );
    (unsafe { c.f() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut final_calls_1)) == (4))
    );
    return 0;
}
unsafe impl DeclaresVirtual for FinalClass {
    unsafe fn f(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut final_calls_1)).prefix_inc();
    }
}
unsafe impl DeclaresVirtual for OverridesAsFinal {
    unsafe fn f(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut final_calls_1)).prefix_inc();
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const base_calls_0);
    std::cell::LazyCell::force(&*&raw const final_calls_1);
}
