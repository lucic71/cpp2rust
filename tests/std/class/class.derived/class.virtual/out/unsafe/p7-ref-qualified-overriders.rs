extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut base_lvalue_calls_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut base_rvalue_calls_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut derived_lvalue_calls_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut derived_rvalue_calls_3: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasRefQualifiedVirtuals {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct OverridesBoth {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct OverridesRvalueOnly {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut both: OverridesBoth = <OverridesBoth>::default();
    let r: *mut HasRefQualifiedVirtuals = &mut both;
    (unsafe { (*r).f_1() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_lvalue_calls_2)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_rvalue_calls_3)) == (0))
    );
    (unsafe { (*r).f_2() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_lvalue_calls_2)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_rvalue_calls_3)) == (1))
    );
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_lvalue_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_rvalue_calls_1)) == (0))
    );
    let mut rvalueOnly: OverridesRvalueOnly = <OverridesRvalueOnly>::default();
    let s: *mut HasRefQualifiedVirtuals = &mut rvalueOnly;
    (unsafe { (*s).f_1() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_lvalue_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_lvalue_calls_2)) == (1))
    );
    (unsafe { (*s).f_2() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_rvalue_calls_1)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_rvalue_calls_3)) == (2))
    );
    return 0;
}
unsafe impl HasRefQualifiedVirtuals for OverridesBoth {
    unsafe fn f_1(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut derived_lvalue_calls_2)).prefix_inc();
    }
    unsafe fn f_2(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut derived_rvalue_calls_3)).prefix_inc();
    }
}
unsafe impl HasRefQualifiedVirtuals for OverridesRvalueOnly {
    unsafe fn f(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut derived_rvalue_calls_3)).prefix_inc();
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const base_lvalue_calls_0);
    std::cell::LazyCell::force(&*&raw const base_rvalue_calls_1);
    std::cell::LazyCell::force(&*&raw const derived_lvalue_calls_2);
    std::cell::LazyCell::force(&*&raw const derived_rvalue_calls_3);
}
