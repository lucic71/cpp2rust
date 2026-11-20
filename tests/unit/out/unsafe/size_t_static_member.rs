extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut c_0: libcc2rs::LazyStorage<usize> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(5_usize) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {}
impl S {
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {}
}
pub static mut table_size_1: libcc2rs::LazyStorage<usize> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(256_usize) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Table_char_ {}
impl Table_char_ {
    pub unsafe fn copy_from(this: *mut Table_char_, _a0: *const Table_char_) {}
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut pa: *const usize = (&raw const (*(*&raw const c_0).get()) as *const usize);
    assert!((((*pa).wrapping_add(1_usize)) == (6_usize)));
    let mut G: *const usize = (&raw const (*(*&raw const table_size_1).get()) as *const usize);
    assert!(((*G) >= (256_usize)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const c_0).get();
    (*&raw const table_size_1).get();
}
