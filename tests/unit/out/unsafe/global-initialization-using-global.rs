extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut first_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub static mut second_1: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(((*(*&raw const first_0).get()) + (1))) });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((*(*&raw const first_0).get()) == (0)));
    assert!(((*(*&raw const second_1).get()) == ((*(*&raw const first_0).get()) + (1))));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const first_0).get();
    (*&raw const second_1).get();
}
