extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut same_name_different_type_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(1) });
pub static mut same_name_same_type_1: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(5) });
pub unsafe fn a_foo_2() -> i32 {
    return (*(*&raw const same_name_different_type_0).get());
}
pub unsafe fn a_bar_3() -> i32 {
    return (*(*&raw const same_name_same_type_1).get());
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { a_foo_2() }) == (1)));
    assert!(((unsafe { b_foo_4() }) == (1_f32)));
    assert!(((unsafe { a_bar_3() }) == (5)));
    assert!(((unsafe { b_bar_5() }) == (6)));
    return 0;
}
pub static mut same_name_different_type_6: libcc2rs::LazyStorage<f32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(1_f32) });
pub static mut same_name_same_type_7: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(6) });
pub unsafe fn b_foo_4() -> f32 {
    return (*(*&raw const same_name_different_type_6).get());
}
pub unsafe fn b_bar_5() -> i32 {
    return (*(*&raw const same_name_same_type_7).get());
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const same_name_different_type_0).get();
    (*&raw const same_name_same_type_1).get();
    (*&raw const same_name_different_type_6).get();
    (*&raw const same_name_same_type_7).get();
}
