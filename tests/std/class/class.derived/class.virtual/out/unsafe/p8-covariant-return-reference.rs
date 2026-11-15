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
pub struct ReturnsBaseReference {
    pub baseValue: i32,
}
impl Default for ReturnsBaseReference {
    fn default() -> Self {
        unsafe { ReturnsBaseReference { baseValue: 0 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ReturnsDerivedReference {
    pub derivedOnly: i32,
}
impl Default for ReturnsDerivedReference {
    fn default() -> Self {
        unsafe { ReturnsDerivedReference { derivedOnly: 0 } }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: ReturnsDerivedReference = <ReturnsDerivedReference>::default();
    let base: *mut ReturnsBaseReference = &mut d;
    (*(unsafe { (*base).self_() })).baseValue = 3;
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)) == (1))
    );
    assert!((((d as ReturnsBaseReference).baseValue) == (3)));
    (*(unsafe { d.self_() })).derivedOnly = 4;
    ((*(unsafe { d.self_() })) as ReturnsBaseReference).baseValue = 5;
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)) == (3))
    );
    assert!(((d.derivedOnly) == (4)));
    assert!((((*base).baseValue) == (5)));
    return 0;
}
unsafe impl ReturnsBaseReference for ReturnsDerivedReference {
    unsafe fn self_(&mut self) -> *mut ReturnsDerivedReference {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)).prefix_inc();
        return &mut (*(self as *mut ReturnsDerivedReference));
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const base_calls_0);
    std::cell::LazyCell::force(&*&raw const derived_calls_1);
}
