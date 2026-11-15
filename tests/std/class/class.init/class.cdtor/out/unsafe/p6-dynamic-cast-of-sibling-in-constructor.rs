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
pub struct VirtualBase {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Sibling {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(*mut Sibling , new, value)]
pub struct CastsSiblingInConstructor {
    pub castSucceeded: bool,
}
impl CastsSiblingInConstructor {
    pub unsafe fn new(mut sibling: *mut Sibling) -> Self {
        let mut this = Self {
            castSucceeded: !((sibling).is_null()),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct MostDerived {}
impl MostDerived {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
}
impl Default for MostDerived {
    fn default() -> Self {
        unsafe { MostDerived::new() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: MostDerived = MostDerived::new();
    return if (d as CastsSiblingInConstructor).castSucceeded {
        1
    } else {
        2
    };
}
pub unsafe fn __cpp2rust_init_globals() {}
