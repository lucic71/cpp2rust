extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut base_non_const_calls_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut base_const_calls_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut derived_const_calls_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut non_const_only_calls_3: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut added_const_calls_4: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasConstAndNonConst {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct OverridesConstOnly {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasNonConstOnly {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct AddsConst {}
impl AddsConst {
    pub unsafe fn f(&self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut added_const_calls_4)).prefix_inc();
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: OverridesConstOnly = <OverridesConstOnly>::default();
    let nonConstView: *mut HasConstAndNonConst = &mut d;
    (unsafe { (*nonConstView).f_1() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut base_non_const_calls_0)) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_const_calls_1)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_const_calls_2)) == (0))
    );
    let constView: *const HasConstAndNonConst = &d;
    (unsafe { (*constView).f_2() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_const_calls_2)) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_non_const_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_const_calls_1)) == (0))
    );
    let mut a: AddsConst = <AddsConst>::default();
    let mut base: *mut HasNonConstOnly = (&mut a as *mut AddsConst);
    (unsafe { (*base).f() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut non_const_only_calls_3)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut added_const_calls_4)) == (0))
    );
    (unsafe { AddsConst::f(&a) });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut non_const_only_calls_3)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut added_const_calls_4)) == (1))
    );
    return 0;
}
unsafe impl HasConstAndNonConst for OverridesConstOnly {
    unsafe fn f(&self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut derived_const_calls_2)).prefix_inc();
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const base_non_const_calls_0);
    std::cell::LazyCell::force(&*&raw const base_const_calls_1);
    std::cell::LazyCell::force(&*&raw const derived_const_calls_2);
    std::cell::LazyCell::force(&*&raw const non_const_only_calls_3);
    std::cell::LazyCell::force(&*&raw const added_const_calls_4);
}
