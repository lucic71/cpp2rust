extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, ptr)]
pub struct S {
    pub r: *const i32,
}
impl S {
    pub unsafe fn new(this: *mut S, x: *const i32) {
        (&raw mut (*this).r).write(x);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let mut __tmp_0: i32 = 5;
        S::new(&raw mut s, { &mut __tmp_0 })
    };
    assert!(((*s.r) == (5)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
