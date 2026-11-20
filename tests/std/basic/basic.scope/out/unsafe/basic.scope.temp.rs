extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut width_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(3) });
pub unsafe fn area_1(mut height: i32) -> i32 {
    return ((5) * (height));
}
pub unsafe fn area_2(mut height: i32) -> i32 {
    return ((7) * (height));
}
pub unsafe fn plain_area_3(mut height: i32) -> i32 {
    return ((*(*&raw const width_0).get()) * (height));
}
pub unsafe fn test_4() {
    assert!(((unsafe { area_1(2,) }) == (10)));
    assert!(((unsafe { area_2(2,) }) == (14)));
    assert!(((unsafe { plain_area_3(2,) }) == (6)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_4() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const width_0).get();
}
