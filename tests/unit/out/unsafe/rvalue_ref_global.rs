extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
static mut __tmp_0: i32 = 5;
pub static mut g_0: libcc2rs::LazyStorage<*mut i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(&mut *&raw mut __tmp_0) });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((*(*(*&raw const g_0).get())) == (5)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const g_0).get();
}
