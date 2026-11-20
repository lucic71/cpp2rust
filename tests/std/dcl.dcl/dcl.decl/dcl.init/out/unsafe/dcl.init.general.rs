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
pub struct p6_1_zero_initialized_pointer_to_member_HasTwoInts {
    pub first: i32,
    pub second: i32,
}
impl p6_1_zero_initialized_pointer_to_member_HasTwoInts {
    pub unsafe fn new_1(this: *mut p6_1_zero_initialized_pointer_to_member_HasTwoInts) {}
    pub unsafe fn copy_from(
        this: *mut p6_1_zero_initialized_pointer_to_member_HasTwoInts,
        _a0: *const p6_1_zero_initialized_pointer_to_member_HasTwoInts,
    ) {
        (&raw mut (*this).first).write((*_a0).first);
        (&raw mut (*this).second).write((*_a0).second);
    }
    pub unsafe fn move_from(
        this: *mut p6_1_zero_initialized_pointer_to_member_HasTwoInts,
        _a0: *mut p6_1_zero_initialized_pointer_to_member_HasTwoInts,
    ) {
        (&raw mut (*this).first).write((*_a0).first);
        (&raw mut (*this).second).write((*_a0).second);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct p6_1_zero_initialized_pointer_to_member_HoldsSelector {
    pub selector: isize,
}
impl p6_1_zero_initialized_pointer_to_member_HoldsSelector {
    pub unsafe fn new_1(this: *mut p6_1_zero_initialized_pointer_to_member_HoldsSelector) {}
    pub unsafe fn copy_from(
        this: *mut p6_1_zero_initialized_pointer_to_member_HoldsSelector,
        _a0: *const p6_1_zero_initialized_pointer_to_member_HoldsSelector,
    ) {
        (&raw mut (*this).selector).write((*_a0).selector);
    }
    pub unsafe fn move_from(
        this: *mut p6_1_zero_initialized_pointer_to_member_HoldsSelector,
        _a0: *mut p6_1_zero_initialized_pointer_to_member_HoldsSelector,
    ) {
        (&raw mut (*this).selector).write((*_a0).selector);
    }
}
impl Default for p6_1_zero_initialized_pointer_to_member_HoldsSelector {
    fn default() -> Self {
        unsafe { p6_1_zero_initialized_pointer_to_member_HoldsSelector { selector: -1_isize } }
    }
}
pub static mut static_pm_0: libcc2rs::LazyStorage<isize> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(-1_isize) });
pub unsafe fn test_1() {
    assert!(((*(*&raw const static_pm_0).get()) == -1_isize));
    assert!(
        ((*(*&raw const static_pm_0).get())
            != (::std::mem::offset_of!(p6_1_zero_initialized_pointer_to_member_HasTwoInts, first)
                as isize))
    );
    let mut value_initialized: isize = -1_isize;
    assert!(((value_initialized) == -1_isize));
    assert!(
        ((value_initialized)
            != (::std::mem::offset_of!(p6_1_zero_initialized_pointer_to_member_HasTwoInts, first)
                as isize))
    );
    let mut holder: p6_1_zero_initialized_pointer_to_member_HoldsSelector =
        ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut holder;
        (&raw mut (*__d).selector).write(-1_isize);
    };
    assert!(((holder.selector) == -1_isize));
    assert!(
        ((holder.selector)
            != (::std::mem::offset_of!(p6_1_zero_initialized_pointer_to_member_HasTwoInts, first)
                as isize))
    );
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_1() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const static_pm_0).get();
}
