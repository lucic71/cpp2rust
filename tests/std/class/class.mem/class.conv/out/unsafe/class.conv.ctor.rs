extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
impl p1_template_constructor_from_own_type_Probe {
    pub unsafe fn new_1(
        this: *mut p1_template_constructor_from_own_type_Probe,
        u: *mut p1_template_constructor_from_own_type_Probe,
    ) {
        (&raw mut (*this).value).write((((*u).value) + (1)));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new_2, value)]
pub struct p1_template_constructor_from_own_type_Probe {
    pub value: i32,
}
impl p1_template_constructor_from_own_type_Probe {
    pub unsafe fn new_2(this: *mut p1_template_constructor_from_own_type_Probe, mut v: i32) {
        (&raw mut (*this).value).write(v);
    }
    pub unsafe fn copy_from(
        this: *mut p1_template_constructor_from_own_type_Probe,
        _a0: *const p1_template_constructor_from_own_type_Probe,
    ) {
        (&raw mut (*this).value).write((*_a0).value);
    }
    pub unsafe fn move_from(
        this: *mut p1_template_constructor_from_own_type_Probe,
        _a0: *mut p1_template_constructor_from_own_type_Probe,
    ) {
        (&raw mut (*this).value).write((*_a0).value);
    }
}
pub unsafe fn test_0() {
    let mut probe: p1_template_constructor_from_own_type_Probe =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p1_template_constructor_from_own_type_Probe::new_2(&raw mut probe, { 22 });
    let mut converted: p1_template_constructor_from_own_type_Probe =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p1_template_constructor_from_own_type_Probe::new_1(&raw mut converted, { &mut probe });
    assert!(((converted.value) == (23)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_0() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
