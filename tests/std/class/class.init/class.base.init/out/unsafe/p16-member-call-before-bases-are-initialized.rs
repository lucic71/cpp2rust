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
pub struct HasIntConstructor {
    pub value: i32,
}
impl HasIntConstructor {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct CallsMemberBeforeBaseIsInitialized {
    pub own: i32,
}
impl CallsMemberBeforeBaseIsInitialized {
    pub unsafe fn readOwn(&mut self) -> i32 {
        return self.own;
    }
    pub unsafe fn new() -> Self {
        let mut this = Self { own: 5 };
        this
    }
}
impl Default for CallsMemberBeforeBaseIsInitialized {
    fn default() -> Self {
        unsafe { CallsMemberBeforeBaseIsInitialized::new() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: CallsMemberBeforeBaseIsInitialized = CallsMemberBeforeBaseIsInitialized::new();
    return (d as HasIntConstructor).value;
}
pub unsafe fn __cpp2rust_init_globals() {}
