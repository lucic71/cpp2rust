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
pub struct pair {
    pub a: i32,
    pub b: i32,
}
impl pair {
    pub unsafe fn new_1(this: *mut pair) {}
    pub unsafe fn copy_from(this: *mut pair, _a0: *const pair) {
        (&raw mut (*this).a).write((*_a0).a);
        (&raw mut (*this).b).write((*_a0).b);
    }
    pub unsafe fn move_from(this: *mut pair, _a0: *mut pair) {
        (&raw mut (*this).a).write((*_a0).a);
        (&raw mut (*this).b).write((*_a0).b);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: i32 = 0_i32;
    x = ((x) + (1));
    let mut p: pair = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut p;
        (&raw mut (*__d).a).write(1);
        (&raw mut (*__d).b).write(((p.a) + (1)));
    };
    return ((x) + (p.b));
}
pub unsafe fn __cpp2rust_init_globals() {}
