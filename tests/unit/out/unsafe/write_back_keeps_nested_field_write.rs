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
pub struct pair {
    pub a: i32,
    pub b: i32,
}
pub unsafe fn bump_0(mut s: *mut pair) -> i32 {
    (*s).b += 10;
    return (*s).b;
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: *mut pair =
        (libcc2rs::calloc_unsafe(1_usize, ::std::mem::size_of::<pair>()) as *mut pair);
    (*s).b = 1;
    (*s).a = (unsafe { bump_0(s) }).clone();
    assert!((((((*s).a) == (11)) as i32) != 0));
    assert!((((((*s).b) == (11)) as i32) != 0));
    libcc2rs::free_unsafe(((s as *mut pair) as *mut ::libc::c_void));
    return 0;
}
