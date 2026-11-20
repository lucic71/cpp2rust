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
pub struct S {
    pub a: i32,
}
impl S {
    pub unsafe fn new_1(this: *mut S) {}
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {
        (&raw mut (*this).a).write((*_a0).a);
    }
    pub unsafe fn move_from(this: *mut S, _a0: *mut S) {
        (&raw mut (*this).a).write((*_a0).a);
    }
}
pub static mut s_0: libcc2rs::LazyStorage<*mut S> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(std::ptr::null_mut()) });
pub static mut file_1: libcc2rs::LazyStorage<*mut ::libc::FILE> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(std::ptr::null_mut()) });
pub static mut size_2: libcc2rs::LazyStorage<usize> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_usize) });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((*(*&raw const s_0).get()).is_null());
    assert!((*(*&raw const file_1).get()).is_null());
    assert!(((*(*&raw const size_2).get()) == (0_usize)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const s_0).get();
    (*&raw const file_1).get();
    (*&raw const size_2).get();
}
