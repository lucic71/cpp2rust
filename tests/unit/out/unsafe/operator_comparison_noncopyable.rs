extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(x: *const S, y: *const S) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_lt_1(x: *const S, y: *const S) -> bool {
    return (((*x).data_) < ((*y).data_));
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[placement]
#[cpp_from(i32, new, value)]
pub struct S {
    data_: i32,
}
impl S {
    pub unsafe fn new(this: *mut S, mut data: i32) {
        (&raw mut (*this).data_).write(data);
    }
    pub unsafe fn move_from(this: *mut S, _a0: *mut S) {
        (&raw mut (*this).data_).write((*_a0).data_);
    }
}
impl std::cmp::Ord for S {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_1(self as *const S, other as *const S) {
                std::cmp::Ordering::Less
            } else if operator_lt_1(other as *const S, self as *const S) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for S {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const S, other as *const S) }
    }
}
impl std::cmp::Eq for S {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: S = ::std::mem::MaybeUninit::uninit().assume_init();
    S::new(&raw mut a, { 1 });
    let mut b: S = ::std::mem::MaybeUninit::uninit().assume_init();
    S::new(&raw mut b, { 2 });
    let mut c: S = ::std::mem::MaybeUninit::uninit().assume_init();
    S::new(&raw mut c, { 1 });
    assert!(
        (unsafe {
            let _x: *const S = &a;
            operator_eq_0(_x, &c)
        })
    );
    assert!(
        (unsafe {
            let _x: *const S = &a;
            operator_lt_1(_x, &b)
        })
    );
    assert!(
        !(unsafe {
            let _x: *const S = &b;
            operator_lt_1(_x, &a)
        })
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
