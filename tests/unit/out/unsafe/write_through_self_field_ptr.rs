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
pub struct S {
    pub x: i32,
    pub p: *mut i32,
    pub self_: *mut S,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = <S>::default();
    s.x = 1;
    s.p = (&raw mut s.x as *mut i32);
    (*s.p) = 5;
    assert!(((((s.x) == (5)) as i32) != 0));
    (*s.p) = ((s.x) + (1));
    assert!(((((s.x) == (6)) as i32) != 0));
    s.self_ = (&raw mut s as *mut S);
    (*s.self_).x = 7;
    assert!(((((s.x) == (7)) as i32) != 0));
    return 0;
}
