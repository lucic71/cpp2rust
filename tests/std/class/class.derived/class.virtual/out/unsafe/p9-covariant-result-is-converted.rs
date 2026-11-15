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
pub struct OtherBase {
    pub other: i32,
}
impl Default for OtherBase {
    fn default() -> Self {
        unsafe { OtherBase { other: 1 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Result {
    pub value: i32,
}
impl Default for Result {
    fn default() -> Self {
        unsafe { Result { value: 2 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct DerivedResult {
    pub extra: i32,
}
impl Default for DerivedResult {
    fn default() -> Self {
        unsafe { DerivedResult { extra: 3 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnsResult {
    pub result: Result,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnsDerivedResult {
    pub derivedResult: DerivedResult,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut f: ReturnsDerivedResult = <ReturnsDerivedResult>::default();
    let mut base: *mut ReturnsResult = (&mut f as *mut ReturnsDerivedResult);
    let mut converted: *mut Result = (unsafe { (*base).get() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)) == (1))
    );
    let mut expected: *mut Result = (&mut f.derivedResult as *mut DerivedResult);
    assert!(((converted) == (expected)));
    assert!((((*converted).value) == (2)));
    (*converted).value = 9;
    assert!((((f.derivedResult as Result).value) == (9)));
    assert!((((f.derivedResult as OtherBase).other) == (1)));
    assert!(((f.derivedResult.extra) == (3)));
    assert!((((f as ReturnsResult).result.value) == (2)));
    let mut notConverted: *mut DerivedResult = (unsafe { f.get() });
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut base_calls_0)) == (0))
            && ((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)) == (2))
    );
    assert!(((notConverted) == (&mut f.derivedResult as *mut DerivedResult)));
    (*notConverted).extra = 7;
    assert!(((f.derivedResult.extra) == (7)));
    return 0;
}
unsafe impl ReturnsResult for ReturnsDerivedResult {
    unsafe fn get(&mut self) -> *mut DerivedResult {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut derived_calls_1)).prefix_inc();
        return (&mut self.derivedResult as *mut DerivedResult);
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const base_calls_0);
    std::cell::LazyCell::force(&*&raw const derived_calls_1);
}
