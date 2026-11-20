extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut s_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(55) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Static_int_ {}
impl Static_int_ {
    pub unsafe fn copy_from(this: *mut Static_int_, _a0: *const Static_int_) {}
}
pub static mut s_1: libcc2rs::LazyStorage<libc::c_char> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write((55 as libc::c_char)) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Static_char_ {}
impl Static_char_ {
    pub unsafe fn copy_from(this: *mut Static_char_, _a0: *const Static_char_) {}
}
pub static mut s_2: libcc2rs::LazyStorage<i64> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(55_i64) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Static_long_ {}
impl Static_long_ {
    pub unsafe fn copy_from(this: *mut Static_long_, _a0: *const Static_long_) {}
}
pub static mut lo_3: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(-1_i32) });
pub static mut hi_4: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(1) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Range_int_ {}
impl Range_int_ {
    pub unsafe fn copy_from(this: *mut Range_int_, _a0: *const Range_int_) {}
}
pub static mut lo_5: libcc2rs::LazyStorage<i64> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write((-2_i32 as i64)) });
pub static mut hi_6: libcc2rs::LazyStorage<i64> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(2_i64) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Range_long_ {}
impl Range_long_ {
    pub unsafe fn copy_from(this: *mut Range_long_, _a0: *const Range_long_) {}
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (*(*&raw const s_0).get()) = 22;
    (*(*&raw const s_1).get()) = (33 as libc::c_char);
    assert!(((*(*&raw const s_0).get()) == (22)));
    assert!((((*(*&raw const s_1).get()) as i32) == (33)));
    assert!(((*(*&raw const s_2).get()) == (55_i64)));
    assert!(((*(*&raw const lo_3).get()) == (-1_i32)) && ((*(*&raw const hi_4).get()) == (1)));
    assert!(
        ((*(*&raw const lo_5).get()) == (-2_i32 as i64))
            && ((*(*&raw const hi_6).get()) == (2_i64))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const s_0).get();
    (*&raw const s_1).get();
    (*&raw const s_2).get();
    (*&raw const lo_3).get();
    (*&raw const hi_4).get();
    (*&raw const lo_5).get();
    (*&raw const hi_6).get();
}
