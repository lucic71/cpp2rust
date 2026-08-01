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
pub struct conn {
    pub first: i32,
    pub port: i32,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: *mut conn = (libcc2rs::malloc_unsafe(::std::mem::size_of::<conn>()) as *mut conn);
    (*c).port = 443;
    let mut p: *mut i32 = (&mut (*c).first as *mut i32);
    (*p) = 1;
    assert!((((((*c).first) == (1)) as i32) != 0));
    assert!((((((*c).port) == (443)) as i32) != 0));
    libcc2rs::free_unsafe(((c as *mut conn) as *mut ::libc::c_void));
    return 0;
}
