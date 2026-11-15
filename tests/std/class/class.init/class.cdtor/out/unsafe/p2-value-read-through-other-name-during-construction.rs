extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct ReadsItselfThroughItsName {
    pub first: i32,
    pub second: i32,
}
impl ReadsItselfThroughItsName {
    pub unsafe fn new(mut a: i32) -> Self {
        let mut this = Self {
            first: a,
            second: (*std::cell::LazyCell::force_mut(&mut *&raw mut object_0)).first,
        };
        this
    }
}
impl ReadsItselfThroughItsName {}
pub static mut object_0: std::cell::LazyCell<ReadsItselfThroughItsName> =
    std::cell::LazyCell::new(|| unsafe { ReadsItselfThroughItsName::new({ 1 }) });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut object_0)).second;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const object_0);
}
