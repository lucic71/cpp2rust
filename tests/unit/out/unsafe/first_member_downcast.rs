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
pub struct base {
    pub next: *mut base,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct derived {
    pub head: base,
    pub value: usize,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: *mut derived =
        (libcc2rs::malloc_unsafe(::std::mem::size_of::<derived>()) as *mut derived);
    assert!((((!((d).is_null())) as i32) != 0));
    (*d).head.next = std::ptr::null_mut();
    (*d).value = 7_usize;
    let mut b: *mut base = (&raw mut (*d).head as *mut base);
    let mut back: *mut derived = (b as *mut derived);
    assert!(((((back) == (d)) as i32) != 0));
    assert!((((((*back).value) == (7_usize)) as i32) != 0));
    libcc2rs::free_unsafe(((back as *mut derived) as *mut ::libc::c_void));
    return 0;
}
