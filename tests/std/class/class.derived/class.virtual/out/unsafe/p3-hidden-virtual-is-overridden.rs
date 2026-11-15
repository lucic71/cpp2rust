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
pub static mut overload_calls_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut overrider_calls_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DeclaresVirtual {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HidesWithOverload {}
impl HidesWithOverload {
    pub unsafe fn f(&mut self, mut _a0: i32) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut overload_calls_1)).prefix_inc();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct OverridesHiddenVirtual {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: OverridesHiddenVirtual = <OverridesHiddenVirtual>::default();
    let mut base: *mut DeclaresVirtual = (&mut d as *mut OverridesHiddenVirtual);
    (unsafe { (*base).f() });
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut overload_calls_1)) == (0)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut overrider_calls_2)) == (1))
    );
    let mut hides: *mut HidesWithOverload = (&mut d as *mut OverridesHiddenVirtual);
    (unsafe { HidesWithOverload::f(&mut (*hides), 1) });
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut overload_calls_1)) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut overrider_calls_2)) == (1))
    );
    (unsafe { (*(hides as *mut DeclaresVirtual)).f() });
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut overload_calls_1)) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut overrider_calls_2)) == (1))
    );
    let mut onlyHides: HidesWithOverload = <HidesWithOverload>::default();
    base = (&mut onlyHides as *mut HidesWithOverload);
    (unsafe { (*base).f() });
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (2))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut overload_calls_1)) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut overrider_calls_2)) == (1))
    );
    return 0;
}
unsafe impl HidesWithOverload for OverridesHiddenVirtual {
    unsafe fn f(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut overrider_calls_2)).prefix_inc();
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const base_calls_0);
    std::cell::LazyCell::force(&*&raw const overload_calls_1);
    std::cell::LazyCell::force(&*&raw const overrider_calls_2);
}
