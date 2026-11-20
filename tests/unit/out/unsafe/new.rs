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
pub struct Pair {
    pub x: i32,
    pub y: i32,
}
impl Pair {
    pub unsafe fn new_1(this: *mut Pair) {}
    pub unsafe fn copy_from(this: *mut Pair, _a0: *const Pair) {
        (&raw mut (*this).x).write((*_a0).x);
        (&raw mut (*this).y).write((*_a0).y);
    }
    pub unsafe fn move_from(this: *mut Pair, _a0: *mut Pair) {
        (&raw mut (*this).x).write((*_a0).x);
        (&raw mut (*this).y).write((*_a0).y);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: *mut i32 = (Box::leak(Box::new(5)) as *mut i32);
    let mut out: i32 = (*x);
    {
        let __p = x;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    assert!(((out) == (5)));
    let mut y: *mut i32 = (Box::leak(Box::new(0_i32)) as *mut i32);
    (*y) = 9;
    assert!(((*y) == (9)));
    {
        let __p = y;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    let mut p: *mut Pair = {
        let __p = Box::into_raw(Box::<Pair>::new_uninit()) as *mut Pair;
        Pair::new_1(__p);
        __p
    };
    (*p).x = 1;
    (*p).y = 2;
    assert!(((((*p).x) + ((*p).y)) == (3)));
    {
        let __p = p;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    let mut nullpointer: *mut i32 = std::ptr::null_mut();
    {
        let __p = nullpointer;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
