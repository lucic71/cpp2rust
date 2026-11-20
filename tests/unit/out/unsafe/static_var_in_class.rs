extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
static mut inner_const_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(1) });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct C {}
impl C {
    pub unsafe fn get(&mut self) -> i32 {
        return (*(*&raw const inner_const_0).get());
    }
    pub unsafe fn new_1(this: *mut C) {}
    pub unsafe fn copy_from(this: *mut C, _a0: *const C) {}
}
pub static mut inner_const_1: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(2) });
pub type anon_3 = u32;
pub const anon_3_kValue: anon_3 = 3;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {}
impl S {
    pub unsafe fn f() -> i32 {
        return (*(*&raw const counter_2).get());
    }
    pub unsafe fn new_1(this: *mut S) {}
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {}
}
pub static mut counter_2: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(10) });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: C = ::std::mem::MaybeUninit::uninit().assume_init();
    C::new_1(&raw mut c);
    assert!(((unsafe { C::get(&mut c,) }) == (1)));
    assert!(((*(*&raw const inner_const_1).get()) == (2)));
    let mut s: S = ::std::mem::MaybeUninit::uninit().assume_init();
    S::new_1(&raw mut s);
    let mut p: *mut S = (&mut s as *mut S);
    assert!(((*(*&raw const inner_const_1).get()) == (2)));
    assert!(((*(*&raw const inner_const_1).get()) == (2)));
    assert!(((anon_3_kValue as i32) == (3)));
    assert!(((anon_3_kValue as i32) == (3)));
    (*(*&raw const counter_2).get()) = 20;
    assert!(((*(*&raw const counter_2).get()) == (20)));
    (*(*&raw const counter_2).get()) += 5;
    assert!(((*(*&raw const counter_2).get()) == (25)));
    assert!(((*(*&raw const counter_2).get()) == (25)));
    assert!(((unsafe { S::f() }) == (25)));
    assert!(((unsafe { S::f() }) == (25)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const inner_const_0).get();
    (*&raw const inner_const_1).get();
    (*&raw const counter_2).get();
}
