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
pub struct payload {
    pub value: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct holder {
    pub first: *mut payload,
    pub second: *mut payload,
    pub count: i32,
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut h: *mut holder =
        (libcc2rs::malloc_unsafe(::std::mem::size_of::<holder>()) as *mut holder);
    assert!((((!((h).is_null())) as i32) != 0));
    (*h).first = (libcc2rs::malloc_unsafe(::std::mem::size_of::<payload>()) as *mut payload);
    (*h).second = (libcc2rs::malloc_unsafe(::std::mem::size_of::<payload>()) as *mut payload);
    assert!((((!(((*h).first).is_null())) as i32) != 0));
    assert!((((!(((*h).second).is_null())) as i32) != 0));
    (*(*h).first).value = 11;
    (*(*h).second).value = 22;
    (*h).count = 2;
    libcc2rs::free_unsafe((((*h).first as *mut payload) as *mut ::libc::c_void));
    (*h).count = 1;
    assert!((((((*h).count) == (1)) as i32) != 0));
    assert!((((((*(*h).second).value) == (22)) as i32) != 0));
    (*h).first = std::ptr::null_mut();
    assert!((((((*h).first).is_null()) as i32) != 0));
    assert!((((((*h).count) == (1)) as i32) != 0));
    libcc2rs::free_unsafe((((*h).second as *mut payload) as *mut ::libc::c_void));
    (*h).second = std::ptr::null_mut();
    (*h).count = 0;
    assert!((((((*h).count) == (0)) as i32) != 0));
    assert!((((((*h).first).is_null()) as i32) != 0));
    libcc2rs::free_unsafe(((h as *mut holder) as *mut ::libc::c_void));
    return 0;
}
