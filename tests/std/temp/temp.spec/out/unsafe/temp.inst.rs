extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p2_reference_does_not_instantiate_Member {
    pub value: i32,
}
impl p2_reference_does_not_instantiate_Member {
    pub unsafe fn new_1(this: *mut p2_reference_does_not_instantiate_Member) {
        (&raw mut (*this).value).write(1);
    }
    pub unsafe fn new_2(
        this: *mut p2_reference_does_not_instantiate_Member,
        _a0: *const p2_reference_does_not_instantiate_Wrapper_int_,
    ) {
        (&raw mut (*this).value).write(2);
    }
}
impl Default for p2_reference_does_not_instantiate_Member {
    fn default() -> Self {
        unsafe { p2_reference_does_not_instantiate_Member { value: 1 } }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct p2_reference_does_not_instantiate_Holder {
    pub member: p2_reference_does_not_instantiate_Member,
}
impl p2_reference_does_not_instantiate_Holder {
    pub unsafe fn new_1(this: *mut p2_reference_does_not_instantiate_Holder) {
        p2_reference_does_not_instantiate_Member::new_1((&raw mut (*this).member));
    }
    pub unsafe fn move_from(
        this: *mut p2_reference_does_not_instantiate_Holder,
        _a0: *mut p2_reference_does_not_instantiate_Holder,
    ) {
        (&raw mut (*this).member).write((*_a0).member);
    }
}
pub unsafe fn test_0() {
    let mut holder: p2_reference_does_not_instantiate_Holder =
        ::std::mem::MaybeUninit::uninit().assume_init();
    p2_reference_does_not_instantiate_Holder::new_1(&raw mut holder);
    assert!(((holder.member.value) == (1)));
    holder.member.value = 3;
    assert!(((holder.member.value) == (3)));
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
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct p2_reference_does_not_instantiate_Wrapper_int_;
pub unsafe fn __cpp2rust_init_globals() {}
