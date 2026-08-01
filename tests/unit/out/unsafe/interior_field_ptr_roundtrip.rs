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
pub struct big {
    pub a: i64,
    pub b: i64,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct outer {
    pub pad: i64,
    pub big: big,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct holder {
    pub p: *mut big,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut o: *mut outer = (libcc2rs::malloc_unsafe(::std::mem::size_of::<outer>()) as *mut outer);
    (*o).pad = 1_i64;
    (*o).big.a = 2_i64;
    (*o).big.b = 3_i64;
    let mut h: *mut holder =
        (libcc2rs::malloc_unsafe(::std::mem::size_of::<holder>()) as *mut holder);
    (*h).p = (&mut (*o).big as *mut big);
    assert!((((((*(*h).p).a) == (2_i64)) as i32) != 0));
    (*(*h).p).b = 9_i64;
    assert!((((((*o).big.b) == (9_i64)) as i32) != 0));
    assert!((((((*o).pad) == (1_i64)) as i32) != 0));
    libcc2rs::free_unsafe(((h as *mut holder) as *mut ::libc::c_void));
    libcc2rs::free_unsafe(((o as *mut outer) as *mut ::libc::c_void));
    return 0;
}
