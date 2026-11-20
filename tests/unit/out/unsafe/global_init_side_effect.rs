extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut total_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct S {}
impl S {
    pub unsafe fn new(this: *mut S, mut x: i32) {
        (*(*&raw const total_0).get()) += x;
    }
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {}
}
pub static mut a_1: libcc2rs::LazyStorage<S> =
    libcc2rs::LazyStorage::new(|__p| unsafe { S::new(__p, { 1 }) });
pub static mut b_2: libcc2rs::LazyStorage<S> =
    libcc2rs::LazyStorage::new(|__p| unsafe { S::new(__p, { 10 }) });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((*(*&raw const total_0).get()) == (11)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const total_0).get();
    (*&raw const a_1).get();
    (*&raw const b_2).get();
}
