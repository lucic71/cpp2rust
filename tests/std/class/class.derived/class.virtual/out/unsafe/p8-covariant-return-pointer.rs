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
pub static mut derived_calls_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnsBasePointer {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ReturnsDerivedPointer {
    pub derivedOnly: i32,
}
impl Default for ReturnsDerivedPointer {
    fn default() -> Self {
        unsafe { ReturnsDerivedPointer { derivedOnly: 0 } }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: ReturnsDerivedPointer = <ReturnsDerivedPointer>::default();
    let mut base: *mut ReturnsBasePointer = (&mut d as *mut ReturnsDerivedPointer);
    let mut fromBase: *mut ReturnsBasePointer = (unsafe { (*base).self_() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)) == (1))
    );
    assert!(((fromBase) == (base)));
    let mut fromDerived: *mut ReturnsDerivedPointer = (unsafe { d.self_() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)) == (2))
    );
    (*fromDerived).derivedOnly = 5;
    assert!(((d.derivedOnly) == (5)));
    let mut b: ReturnsBasePointer = <ReturnsBasePointer>::default();
    assert!(((unsafe { b.self_() }) == (&mut b as *mut ReturnsBasePointer)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (1))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)) == (2))
    );
    return 0;
}
unsafe impl ReturnsBasePointer for ReturnsDerivedPointer {
    unsafe fn self_(&mut self) -> *mut ReturnsDerivedPointer {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)).prefix_inc();
        return (self as *mut ReturnsDerivedPointer);
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const base_calls_0);
    std::cell::LazyCell::force(&*&raw const derived_calls_1);
}
