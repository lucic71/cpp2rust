extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn a_0() -> i32 {
    return (*(*&raw const i_1).get());
}
static mut i_1: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(1) });
pub unsafe fn b_2() -> i32 {
    return (*(*&raw const i_3).get());
}
static mut i_3: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(2) });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { a_0() }) == (1)));
    assert!(((unsafe { b_2() }) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
