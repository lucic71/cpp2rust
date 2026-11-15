extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct HasMember {
    pub value: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct HasNonTrivialConstructor {}
impl HasNonTrivialConstructor {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this.value = 7;
        this
    }
}
impl Default for HasNonTrivialConstructor {
    fn default() -> Self {
        unsafe { HasNonTrivialConstructor::new() }
    }
}
pub static mut formedBeforeConstruction_1: std::cell::LazyCell<*mut i32> =
    std::cell::LazyCell::new(|| unsafe {
        (&mut ((*std::cell::LazyCell::force_mut(&mut *&raw mut object_0)) as HasMember).value
            as *mut i32)
    });
pub static mut object_0: std::cell::LazyCell<HasNonTrivialConstructor> =
    std::cell::LazyCell::new(|| unsafe { HasNonTrivialConstructor::new() });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    return (*(*std::cell::LazyCell::force_mut(&mut *&raw mut formedBeforeConstruction_1)));
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const formedBeforeConstruction_1);
    std::cell::LazyCell::force(&*&raw const object_0);
}
