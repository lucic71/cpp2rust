extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut failed_casts_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut successful_casts_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct CastsToDerived {}
impl CastsToDerived {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        (unsafe { castToDerived_2(&raw mut this) });
        this
    }
    unsafe fn destructor(&mut self) {
        (unsafe { castToDerived_2((self as *mut CastsToDerived)) });
    }
}
impl Default for CastsToDerived {
    fn default() -> Self {
        unsafe { CastsToDerived::new() }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct DerivesFromCastsToDerived {}
impl DerivesFromCastsToDerived {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        (unsafe { castToDerived_2(&raw mut this) });
        this
    }
    unsafe fn destructor(&mut self) {
        (unsafe { castToDerived_2((self as *mut DerivesFromCastsToDerived)) });
    }
}
impl Default for DerivesFromCastsToDerived {
    fn default() -> Self {
        unsafe { DerivesFromCastsToDerived::new() }
    }
}
pub unsafe fn castToDerived_2(mut object: *mut CastsToDerived) {
    if (object).is_null() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut failed_casts_0)).prefix_inc();
    } else {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut successful_casts_1)).prefix_inc();
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        let mut d: DerivesFromCastsToDerived = DerivesFromCastsToDerived::new();
        let _dtor_d = std::mem::DropGuard::new(&raw mut d, |__p| unsafe { (*__p).destructor() });
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut failed_casts_0)) == (1))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut successful_casts_1)) == (1))
        );
        (unsafe { castToDerived_2((&mut d as *mut DerivesFromCastsToDerived)) });
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut failed_casts_0)) == (1))
                && ((*std::cell::LazyCell::force_mut(&mut *&raw mut successful_casts_1)) == (2))
        );
    }
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut failed_casts_0)) == (2))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut successful_casts_1)) == (3))
    );
    return 0;
}
unsafe impl CastsToDerived for DerivesFromCastsToDerived {}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const failed_casts_0);
    std::cell::LazyCell::force(&*&raw const successful_casts_1);
}
