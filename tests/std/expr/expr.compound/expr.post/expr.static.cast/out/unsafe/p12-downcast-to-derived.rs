extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut derived_only_calls_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Base {
    pub base: i32,
}
impl Default for Base {
    fn default() -> Self {
        unsafe { Base { base: 1 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Derived {
    pub derived: i32,
}
impl Derived {
    pub unsafe fn derivedOnly(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut derived_only_calls_0)).prefix_inc();
    }
}
impl Default for Derived {
    fn default() -> Self {
        unsafe { Derived { derived: 2 } }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: Derived = <Derived>::default();
    let mut base: *mut Base = (&mut d as *mut Derived);
    let mut derived: *mut Derived = (base as *mut Derived);
    assert!(((derived) == (&mut d as *mut Derived)));
    assert!((((*derived).derived) == (2)));
    (unsafe { Derived::derivedOnly(&mut (*derived)) });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut derived_only_calls_0)) == (1)));
    (*derived).derived = 5;
    assert!(((d.derived) == (5)));
    assert!((((d as Base).base) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const derived_only_calls_0);
}
