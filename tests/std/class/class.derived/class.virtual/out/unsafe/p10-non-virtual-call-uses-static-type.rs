extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut base_overridden_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut base_hidden_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut base_non_virtual_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut derived_overridden_3: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut derived_overload_4: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut derived_non_virtual_5: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DeclaresFunctions {}
impl DeclaresFunctions {
    pub unsafe fn nonVirtual(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut base_non_virtual_2)).prefix_inc();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RedeclaresFunctions {}
impl RedeclaresFunctions {
    pub unsafe fn hiddenByOverload(&mut self, mut _a0: i32) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut derived_overload_4)).prefix_inc();
    }
    pub unsafe fn nonVirtual(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut derived_non_virtual_5)).prefix_inc();
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: RedeclaresFunctions = <RedeclaresFunctions>::default();
    let mut base: *mut DeclaresFunctions = (&mut d as *mut RedeclaresFunctions);
    (unsafe { (*base).overridden() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_overridden_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_overridden_3)) == (1))
    );
    (unsafe { (*base).hiddenByOverload() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_hidden_1)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_overload_4)) == (0))
    );
    (unsafe { DeclaresFunctions::nonVirtual(&mut (*base)) });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_non_virtual_2)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_non_virtual_5)) == (0))
    );
    let mut derived: *mut RedeclaresFunctions = (&mut d as *mut RedeclaresFunctions);
    (unsafe { (*derived).overridden() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_overridden_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_overridden_3)) == (2))
    );
    (unsafe { RedeclaresFunctions::hiddenByOverload(&mut (*derived), 0) });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_hidden_1)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_overload_4)) == (1))
    );
    (unsafe { RedeclaresFunctions::nonVirtual(&mut (*derived)) });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_non_virtual_2)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_non_virtual_5)) == (1))
    );
    return 0;
}
unsafe impl DeclaresFunctions for RedeclaresFunctions {
    unsafe fn overridden(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut derived_overridden_3)).prefix_inc();
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const base_overridden_0);
    std::cell::LazyCell::force(&*&raw const base_hidden_1);
    std::cell::LazyCell::force(&*&raw const base_non_virtual_2);
    std::cell::LazyCell::force(&*&raw const derived_overridden_3);
    std::cell::LazyCell::force(&*&raw const derived_overload_4);
    std::cell::LazyCell::force(&*&raw const derived_non_virtual_5);
}
