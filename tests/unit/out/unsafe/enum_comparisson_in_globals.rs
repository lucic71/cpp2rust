extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type E = u32;
pub const E_A: E = 0;
pub const E_B: E = 1;
pub static mut global_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write((((E_A as i32) != (E_B as i32)) as i32)) });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((*(*&raw const global_0).get()) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const global_0).get();
}
