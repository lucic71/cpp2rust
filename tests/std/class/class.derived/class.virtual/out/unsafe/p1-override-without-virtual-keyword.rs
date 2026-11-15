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
pub struct OverridesWithoutKeyword {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct OverridesAgain {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: OverridesAgain = <OverridesAgain>::default();
    let mut first: *mut OverridesWithoutKeyword = (&mut d as *mut OverridesAgain);
    (unsafe { (*first).f() });
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_1)) == (0)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_2)) == (1))
    );
    let mut base: *mut DeclaresVirtual = (&mut d as *mut OverridesAgain);
    (unsafe { (*base).f() });
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_1)) == (0)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_2)) == (2))
    );
    let mut m: OverridesWithoutKeyword = <OverridesWithoutKeyword>::default();
    base = (&mut m as *mut OverridesWithoutKeyword);
    (unsafe { (*base).f() });
    assert!(
        (((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_1)) == (1)))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_2)) == (2))
    );
    return 0;
}
unsafe impl OverridesWithoutKeyword for OverridesAgain {
    unsafe fn f(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut second_calls_2)).prefix_inc();
    }
}
unsafe impl DeclaresVirtual for OverridesWithoutKeyword {
    unsafe fn f(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut first_calls_1)).prefix_inc();
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const base_calls_0);
    std::cell::LazyCell::force(&*&raw const first_calls_1);
    std::cell::LazyCell::force(&*&raw const second_calls_2);
}
