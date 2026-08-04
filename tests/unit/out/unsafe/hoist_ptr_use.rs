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
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct outer {
    pub in_: inner,
    pub total: i32,
}
pub unsafe fn read_total_0(mut o: *mut outer) -> i32 {
    return (*o).total;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut o: outer = outer {
        in_: inner { x: 1, y: 2 },
        total: 10,
    };
    let mut p: *mut outer = (&mut o as *mut outer);
    let mut q: *mut outer = (&mut o as *mut outer);
    (*p).total = (((*q).in_.x) + ((*q).in_.y));
    assert!(((((o.total) == (3)) as i32) != 0));
    let mut ip: *mut inner = (&mut (*p).in_ as *mut inner);
    (*ip).x = (((*p).total) + (1));
    assert!(((((o.in_.x) == (4)) as i32) != 0));
    (*p).total += (*q).in_.x;
    assert!(((((o.total) == (7)) as i32) != 0));
    (*p).in_.y = (unsafe { read_total_0(q) }).clone();
    assert!(((((o.in_.y) == (7)) as i32) != 0));
    let mut h: *mut outer = (libcc2rs::malloc_unsafe(::std::mem::size_of::<outer>()) as *mut outer);
    let mut ha: *mut outer = h;
    (*h).total = 5;
    (*h).in_.x = 1;
    (*ha).total = (((*h).total) + ((*ha).in_.x));
    assert!((((((*h).total) == (6)) as i32) != 0));
    libcc2rs::free_unsafe(((h as *mut outer) as *mut ::libc::c_void));
    return 0;
}
