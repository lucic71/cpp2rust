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
pub struct DeclaresABC {
    pub a: i32,
    pub b: i32,
    pub c: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RedeclaresB {
    pub b: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RedeclaresC {
    pub c: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: RedeclaresC = <RedeclaresC>::default();
    (d as DeclaresABC).a = 1;
    (d as DeclaresABC).b = 2;
    (d as DeclaresABC).c = 3;
    (d as RedeclaresB).b = 20;
    d.c = 30;
    assert!((((d as DeclaresABC).a) == (1)));
    assert!((((d as RedeclaresB).b) == (20)));
    assert!(((d.c) == (30)));
    assert!((((d as DeclaresABC).b) == (2)));
    assert!((((d as DeclaresABC).c) == (3)));
    let asRedeclaresB: *mut RedeclaresB = &mut d;
    assert!(((((*asRedeclaresB) as DeclaresABC).a) == (1)));
    assert!((((*asRedeclaresB).b) == (20)));
    assert!(((((*asRedeclaresB) as DeclaresABC).c) == (3)));
    let asDeclaresABC: *mut DeclaresABC = &mut d;
    assert!((((*asDeclaresABC).a) == (1)));
    assert!((((*asDeclaresABC).b) == (2)));
    assert!((((*asDeclaresABC).c) == (3)));
    (*asDeclaresABC).b = 4;
    ((*asRedeclaresB) as DeclaresABC).c = 5;
    assert!((((d as RedeclaresB).b) == (20)));
    assert!(((d.c) == (30)));
    assert!((((d as DeclaresABC).b) == (4)));
    assert!((((d as DeclaresABC).c) == (5)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
