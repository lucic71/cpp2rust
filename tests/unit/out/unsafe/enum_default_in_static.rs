extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Mode = u32;
pub const Mode_MODE_NONE: Mode = 0;
pub const Mode_MODE_ONE: Mode = 1;
pub const Mode_MODE_TWO: Mode = 2;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Config {
    pub count: i32,
    pub mode: Mode,
}
pub static mut config_0: libcc2rs::LazyStorage<Config> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(<Config>::default()) });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!((((*(*&raw const config_0).get()).count) == (0)));
    assert!((((*(*&raw const config_0).get()).mode as u32) == ((Mode_MODE_NONE as i32) as u32)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const config_0).get();
}
