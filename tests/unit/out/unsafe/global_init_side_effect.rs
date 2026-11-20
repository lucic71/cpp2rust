extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut total_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[placement]
#[cpp_from(i32, new, value)]
pub struct S {}
impl S {
    pub unsafe fn new(this: *mut S, mut x: i32) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_0)) += x;
    }
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {}
}
pub static mut a_1: std::cell::LazyCell<S> = std::cell::LazyCell::new(|| unsafe {
    {
        let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
        S::new(&raw mut __v, { 1 });
        __v
    }
});
pub static mut b_2: std::cell::LazyCell<S> = std::cell::LazyCell::new(|| unsafe {
    {
        let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
        S::new(&raw mut __v, { 10 });
        __v
    }
});
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut total_0)) == (11)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const total_0);
    std::cell::LazyCell::force(&*&raw const a_1);
    std::cell::LazyCell::force(&*&raw const b_2);
}
