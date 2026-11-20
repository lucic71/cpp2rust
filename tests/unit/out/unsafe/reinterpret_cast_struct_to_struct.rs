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
pub struct Point {
    pub x: u32,
    pub y: u32,
}
impl Point {
    pub unsafe fn new_1(this: *mut Point) {}
    pub unsafe fn copy_from(this: *mut Point, _a0: *const Point) {
        (&raw mut (*this).x).write((*_a0).x);
        (&raw mut (*this).y).write((*_a0).y);
    }
    pub unsafe fn move_from(this: *mut Point, _a0: *mut Point) {
        (&raw mut (*this).x).write((*_a0).x);
        (&raw mut (*this).y).write((*_a0).y);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Pair {
    pub first: u32,
    pub second: u32,
}
impl Pair {
    pub unsafe fn new_1(this: *mut Pair) {}
    pub unsafe fn copy_from(this: *mut Pair, _a0: *const Pair) {
        (&raw mut (*this).first).write((*_a0).first);
        (&raw mut (*this).second).write((*_a0).second);
    }
    pub unsafe fn move_from(this: *mut Pair, _a0: *mut Pair) {
        (&raw mut (*this).first).write((*_a0).first);
        (&raw mut (*this).second).write((*_a0).second);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut pt: Point = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut pt;
        (&raw mut (*__d).x).write(10_u32);
        (&raw mut (*__d).y).write(20_u32);
    };
    let mut pair: *mut Pair = ((&mut pt as *mut Point) as *mut Pair);
    assert!((((*pair).first) == (10_u32)));
    assert!((((*pair).second) == (20_u32)));
    (*pair).first = 42_u32;
    assert!(((pt.x) == (42_u32)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
