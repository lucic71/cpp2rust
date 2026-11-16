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
pub struct Intermediate {
    pub intermediate: i32,
}
impl Default for Intermediate {
    fn default() -> Self {
        unsafe { Intermediate { intermediate: 2 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct MostDerived {
    pub mostDerived: i32,
}
impl Default for MostDerived {
    fn default() -> Self {
        unsafe { MostDerived { mostDerived: 3 } }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: MostDerived = <MostDerived>::default();
    let mut base: *mut Base = (&mut d as *mut MostDerived);
    let mut toMostDerived: *mut MostDerived = base;
    assert!(((toMostDerived) == (&mut d as *mut MostDerived)));
    assert!((((*toMostDerived).mostDerived) == (3)));
    let mut toIntermediate: *mut Intermediate = base;
    assert!(((toIntermediate) == (&mut d as *mut MostDerived)));
    assert!((((*toIntermediate).intermediate) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
