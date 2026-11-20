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
pub struct HasTwoInts {
    pub first: i32,
    pub second: i32,
}
impl HasTwoInts {
    pub unsafe fn copy_from(this: *mut HasTwoInts, _a0: *const HasTwoInts) {
        (&raw mut (*this).first).write((*_a0).first);
        (&raw mut (*this).second).write((*_a0).second);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut pm: isize = -1_isize;
    let mut obj: HasTwoInts = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut obj;
        (&raw mut (*__d).first).write(3);
        (&raw mut (*__d).second).write(4);
    };
    return (*(&mut obj as *mut HasTwoInts)
        .wrapping_byte_offset(pm)
        .cast::<i32>());
}
pub unsafe fn __cpp2rust_init_globals() {}
