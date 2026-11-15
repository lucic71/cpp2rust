extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct HasValue {
    pub value: i32,
}
impl Default for HasValue {
    fn default() -> Self {
        unsafe { HasValue { value: 3 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(*mut HasValue , new, value)]
pub struct StoresBasePointer {
    pub stored: *mut HasValue,
}
impl StoresBasePointer {
    pub unsafe fn new(mut base: *mut HasValue) -> Self {
        let mut this = Self { stored: base };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConvertsThisTooEarly {}
impl ConvertsThisTooEarly {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
}
impl Default for ConvertsThisTooEarly {
    fn default() -> Self {
        unsafe { ConvertsThisTooEarly::new() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: ConvertsThisTooEarly = ConvertsThisTooEarly::new();
    return (*(d as StoresBasePointer).stored).value;
}
pub unsafe fn __cpp2rust_init_globals() {}
