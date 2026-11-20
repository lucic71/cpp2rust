extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut global_calls_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub static mut inner_calls_1: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub unsafe fn helper_2() -> i32 {
    (*(*&raw const global_calls_0).get()).prefix_inc();
    return 100;
}
pub unsafe fn run_3() -> i32 {
    return (unsafe { helper_4() });
}
pub unsafe fn helper_4() -> i32 {
    (*(*&raw const inner_calls_1).get()).prefix_inc();
    return 9;
}
pub unsafe fn test_5() {
    assert!(((unsafe { run_3() }) == (9)));
    assert!(((*(*&raw const inner_calls_1).get()) == (1)));
    assert!(((*(*&raw const global_calls_0).get()) == (0)));
}
pub static mut global_calls_6: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub static mut inner_calls_7: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub unsafe fn base_8() -> i32 {
    (*(*&raw const global_calls_6).get()).prefix_inc();
    return 100;
}
pub unsafe fn base_9() -> i32 {
    (*(*&raw const inner_calls_7).get()).prefix_inc();
    return 7;
}
pub unsafe fn twice_11() -> i32 {
    return ((unsafe { base_9() }) * (2));
}
pub static mut stored_10: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(((unsafe { base_9() }) + (1))) });
pub unsafe fn test_12() {
    assert!(((*(*&raw const stored_10).get()) == (8)));
    assert!(((*(*&raw const inner_calls_7).get()) == (1)));
    assert!(((unsafe { twice_11() }) == (14)));
    assert!(((*(*&raw const inner_calls_7).get()) == (2)));
    assert!(((*(*&raw const global_calls_6).get()) == (0)));
}
pub static mut count_13: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(4) });
pub static mut count_14: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(100) });
pub unsafe fn next_15() -> i32 {
    return (*(*(*&raw const count_13).get()).prefix_inc());
}
pub unsafe fn test_16() {
    assert!(((unsafe { next_15() }) == (5)));
    assert!(((*(*&raw const count_13).get()) == (5)));
    assert!(((*(*&raw const count_14).get()) == (100)));
    assert!(((*(*&raw const count_14).get()) == (100)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_5() });
    (unsafe { test_12() });
    (unsafe { test_16() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const global_calls_0).get();
    (*&raw const inner_calls_1).get();
    (*&raw const global_calls_6).get();
    (*&raw const inner_calls_7).get();
    (*&raw const stored_10).get();
    (*&raw const count_13).get();
    (*&raw const count_14).get();
}
