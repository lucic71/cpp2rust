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
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ReturnsPointerToConst {
    pub value: i32,
}
impl Default for ReturnsPointerToConst {
    fn default() -> Self {
        unsafe { ReturnsPointerToConst { value: 1 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnsPointerToNonConst {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: ReturnsPointerToNonConst = <ReturnsPointerToNonConst>::default();
    let mut base: *mut ReturnsPointerToConst = (&mut d as *mut ReturnsPointerToNonConst);
    let mut fromBase: *const ReturnsPointerToConst = (unsafe { (*base).self_() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)) == (1))
    );
    assert!((((*fromBase).value) == (1)));
    (*((unsafe { d.self_() }) as *mut ReturnsPointerToConst)).value = 2;
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)) == (2))
    );
    assert!((((*fromBase).value) == (2)));
    return 0;
}
unsafe impl ReturnsPointerToConst for ReturnsPointerToNonConst {
    unsafe fn self_(&mut self) -> *mut ReturnsPointerToNonConst {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)).prefix_inc();
        return (self as *mut ReturnsPointerToNonConst);
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const base_calls_0);
    std::cell::LazyCell::force(&*&raw const derived_calls_1);
}
