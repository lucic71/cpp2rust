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
pub static mut sibling_calls_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut most_derived_calls_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct VirtualBase {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SiblingOverrider {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(*mut SiblingOverrider , new, value)]
pub struct CallsThroughSiblingInConstructor {}
impl CallsThroughSiblingInConstructor {
    pub unsafe fn new(mut sibling: *mut SiblingOverrider) -> Self {
        let mut this = Self {};
        (unsafe { (*sibling).f() });
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct MostDerived {}
impl MostDerived {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
}
impl Default for MostDerived {
    fn default() -> Self {
        unsafe { MostDerived::new() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: MostDerived = MostDerived::new();
    return (((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0))
        + ((2) * (*std::cell::LazyCell::force_mut(&mut *&raw mut sibling_calls_1))))
        + ((4) * (*std::cell::LazyCell::force_mut(&mut *&raw mut most_derived_calls_2))));
}
unsafe impl SiblingOverrider for MostDerived {
    unsafe fn f(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut most_derived_calls_2)).prefix_inc();
    }
}
unsafe impl VirtualBase for SiblingOverrider {
    unsafe fn f(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut sibling_calls_1)).prefix_inc();
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const base_calls_0);
    std::cell::LazyCell::force(&*&raw const sibling_calls_1);
    std::cell::LazyCell::force(&*&raw const most_derived_calls_2);
}
