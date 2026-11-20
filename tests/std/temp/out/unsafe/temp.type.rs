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
pub struct p2_6_pointer_to_member_argument_HasTwoInts {
    pub first: i32,
    pub second: i32,
}
impl p2_6_pointer_to_member_argument_HasTwoInts {
    pub unsafe fn new_1(this: *mut p2_6_pointer_to_member_argument_HasTwoInts) {}
    pub unsafe fn copy_from(
        this: *mut p2_6_pointer_to_member_argument_HasTwoInts,
        _a0: *const p2_6_pointer_to_member_argument_HasTwoInts,
    ) {
        (&raw mut (*this).first).write((*_a0).first);
        (&raw mut (*this).second).write((*_a0).second);
    }
    pub unsafe fn move_from(
        this: *mut p2_6_pointer_to_member_argument_HasTwoInts,
        _a0: *mut p2_6_pointer_to_member_argument_HasTwoInts,
    ) {
        (&raw mut (*this).first).write((*_a0).first);
        (&raw mut (*this).second).write((*_a0).second);
    }
}
pub static mut count_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_first_
{}
impl p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_first_ { pub unsafe fn new_1 ( this: *mut p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_first_, ) { } pub unsafe fn copy_from ( this: *mut p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_first_, _a0 : *const p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_first_ , ) { } pub unsafe fn move_from ( this: *mut p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_first_, _a0 : *mut p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_first_ , ) { } }
pub static mut count_1: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_second_
{}
impl p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_second_ { pub unsafe fn new_1 ( this: *mut p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_second_, ) { } pub unsafe fn copy_from ( this: *mut p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_second_, _a0 : *const p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_second_ , ) { } pub unsafe fn move_from ( this: *mut p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_second_, _a0 : *mut p2_6_pointer_to_member_argument_CountsPerMember_refp2_6_pointer_to_member_argument_HasTwoInts_second_ , ) { } }
pub static mut count_2: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct p2_6_pointer_to_member_argument_CountsPerMember_nullptr_ {}
impl p2_6_pointer_to_member_argument_CountsPerMember_nullptr_ {
    pub unsafe fn new_1(this: *mut p2_6_pointer_to_member_argument_CountsPerMember_nullptr_) {}
    pub unsafe fn copy_from(
        this: *mut p2_6_pointer_to_member_argument_CountsPerMember_nullptr_,
        _a0: *const p2_6_pointer_to_member_argument_CountsPerMember_nullptr_,
    ) {
    }
    pub unsafe fn move_from(
        this: *mut p2_6_pointer_to_member_argument_CountsPerMember_nullptr_,
        _a0: *mut p2_6_pointer_to_member_argument_CountsPerMember_nullptr_,
    ) {
    }
}
pub unsafe fn test_3() {
    (*(*&raw const count_0).get()).prefix_inc();
    assert!(((*(*&raw const count_0).get()) == (1)));
    assert!(((*(*&raw const count_1).get()) == (0)));
    assert!(((*(*&raw const count_2).get()) == (0)));
    (*(*&raw const count_1).get()) += 2;
    assert!(((*(*&raw const count_0).get()) == (1)));
    assert!(((*(*&raw const count_1).get()) == (2)));
    assert!(((*(*&raw const count_2).get()) == (0)));
    (*(*&raw const count_2).get()) += 3;
    assert!(((*(*&raw const count_0).get()) == (1)));
    assert!(((*(*&raw const count_1).get()) == (2)));
    assert!(((*(*&raw const count_2).get()) == (3)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_3() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const count_0).get();
    (*&raw const count_1).get();
    (*&raw const count_2).get();
}
