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
pub static mut overrider_calls_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DeclaresVirtual {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct InheritsWithoutOverriding {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Overrides {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasBothPaths {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: HasBothPaths = <HasBothPaths>::default();
    let mut notOverridden: *mut InheritsWithoutOverriding = (&mut d as *mut HasBothPaths);
    let mut overridden: *mut Overrides = (&mut d as *mut HasBothPaths);
    (unsafe { (*(notOverridden as *mut DeclaresVirtual)).f() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut overrider_calls_1)) == (0))
    );
    (unsafe { (*overridden).f() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut overrider_calls_1)) == (1))
    );
    let mut baseOfNotOverridden: *mut DeclaresVirtual = notOverridden;
    let mut baseOfOverridden: *mut DeclaresVirtual = overridden;
    (unsafe { (*baseOfNotOverridden).f() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (2))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut overrider_calls_1)) == (1))
    );
    (unsafe { (*baseOfOverridden).f() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (2))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut overrider_calls_1)) == (2))
    );
    return 0;
}
unsafe impl DeclaresVirtual for Overrides {
    unsafe fn f(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut overrider_calls_1)).prefix_inc();
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const base_calls_0);
    std::cell::LazyCell::force(&*&raw const overrider_calls_1);
}
