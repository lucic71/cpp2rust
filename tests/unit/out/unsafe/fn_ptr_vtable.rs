extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Vtable {
    pub create: Option<unsafe fn(i32) -> *mut ::libc::c_void>,
    pub get: Option<unsafe fn(*mut ::libc::c_void) -> i32>,
    pub destroy: Option<unsafe fn(*mut ::libc::c_void)>,
}
impl Vtable {
    pub unsafe fn new_1(this: *mut Vtable) {}
    pub unsafe fn copy_from(this: *mut Vtable, _a0: *const Vtable) {
        (&raw mut (*this).create).write(((*_a0).create));
        (&raw mut (*this).get).write(((*_a0).get));
        (&raw mut (*this).destroy).write(((*_a0).destroy));
    }
    pub unsafe fn move_from(this: *mut Vtable, _a0: *mut Vtable) {
        (&raw mut (*this).create).write(((*_a0).create));
        (&raw mut (*this).get).write(((*_a0).get));
        (&raw mut (*this).destroy).write(((*_a0).destroy));
    }
}
pub static mut storage_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0_i32) });
pub unsafe fn int_create_1(mut val: i32) -> *mut ::libc::c_void {
    (*(*&raw const storage_0).get()) = val;
    return ((&raw mut (*(*&raw const storage_0).get()) as *mut i32) as *mut ::libc::c_void);
}
pub unsafe fn int_get_2(mut p: *mut ::libc::c_void) -> i32 {
    return (*(p as *mut i32));
}
pub unsafe fn int_destroy_3(mut p: *mut ::libc::c_void) {
    (*(p as *mut i32)) = 0;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut vt: Vtable = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut vt;
        (&raw mut (*__d).create).write(Some(int_create_1));
        (&raw mut (*__d).get).write(Some(int_get_2));
        (&raw mut (*__d).destroy).write(Some(int_destroy_3));
    };
    assert!(!((vt.create).is_none()));
    assert!(!((vt.get).is_none()));
    assert!(!((vt.destroy).is_none()));
    let mut obj: *mut ::libc::c_void = (unsafe { (vt.create).unwrap()(42) });
    assert!(((unsafe { (vt.get).unwrap()(obj,) }) == (42)));
    (unsafe { (vt.destroy).unwrap()(obj) });
    assert!(((*(*&raw const storage_0).get()) == (0)));
    (vt.get) = None;
    assert!((vt.get).is_none());
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const storage_0).get();
}
