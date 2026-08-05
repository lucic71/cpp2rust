extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct inner {
    pub a: i64,
    pub b: i64,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct outer {
    pub in_: inner,
    pub tag: i64,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct holder {
    pub words: *mut i64,
    pub field: *mut inner,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut o: *mut outer = (libcc2rs::malloc_unsafe(::std::mem::size_of::<outer>()) as *mut outer);
    (*o).tag = 7_i64;
    let mut h: *mut holder =
        (libcc2rs::malloc_unsafe(::std::mem::size_of::<holder>()) as *mut holder);
    (*h).words =
        (libcc2rs::malloc_unsafe((2_usize).wrapping_mul((::std::mem::size_of::<i64>() as usize)))
            as *mut i64);
    (*h).field = (&raw mut (*o).in_ as *mut inner);
    (*(*h).words.offset(((0) as isize))) = 11_i64;
    (*(*h).words.offset(((1) as isize))) = 22_i64;
    (*(*h).field).a = 33_i64;
    assert!(((((*(*h).words.offset(((0) as isize))) == (11_i64)) as i32) != 0));
    assert!(((((*(*h).words.offset(((1) as isize))) == (22_i64)) as i32) != 0));
    assert!((((((*(*h).field).a) == (33_i64)) as i32) != 0));
    assert!((((((*o).tag) == (7_i64)) as i32) != 0));
    libcc2rs::free_unsafe((((*h).words as *mut i64) as *mut ::libc::c_void));
    libcc2rs::free_unsafe(((h as *mut holder) as *mut ::libc::c_void));
    libcc2rs::free_unsafe(((o as *mut outer) as *mut ::libc::c_void));
    return 0;
}
