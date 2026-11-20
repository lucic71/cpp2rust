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
#[placement]
pub struct S {
    pub v: i32,
}
impl S {
    pub unsafe fn get(&self) -> i32 {
        return self.v;
    }
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
pub unsafe trait Base {
    unsafe fn apply(&mut self, x: i32) -> i32;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
#[placement]
pub struct Derived {
    pub factor: i32,
}
impl Derived {
    pub unsafe fn copy_from(this: *mut Derived, _a0: *const Derived) {
        (&raw mut (*this).factor).write((*_a0).factor);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Pair {
    pub first: i32,
    pub second: i32,
}
pub unsafe fn pair_diff_0(mut p: *const Pair) -> i32 {
    return (((*p).first) - ((*p).second));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = ::std::mem::MaybeUninit::uninit().assume_init();
    S::new(&raw mut s, { 1 });
    assert!(((unsafe { S::get(&s,) }) == (1)));
    (unsafe { S::set(&mut s, 4) });
    assert!(((unsafe { S::get(&s,) }) == (4)));
    assert!(((unsafe { S::add(&mut s, 2,) }) == (6)));
    let mut derived: Derived = ::std::mem::MaybeUninit::uninit().assume_init();
    Derived::new(&raw mut derived, { 3 });
    let mut base: *mut dyn Base = (&mut derived as *mut Derived);
    assert!(((unsafe { (*base).apply(5,) }) == (15)));
    let mut pair: Pair = Pair {
        first: 7,
        second: 3,
    };
    assert!(((unsafe { pair_sum_1((&mut pair as *mut Pair).cast_const(),) }) == (10)));
    assert!(((unsafe { pair_diff_0((&mut pair as *mut Pair).cast_const(),) }) == (4)));
    return 0;
}
impl S {
    pub unsafe fn new(this: *mut S, mut x: i32) {
        (&raw mut (*this).v).write(x);
    }
}
impl Derived {
    pub unsafe fn new(this: *mut Derived, mut factor: i32) {
        (&raw mut (*this).factor).write(factor);
    }
}
impl S {}
impl S {
    pub unsafe fn destructor(&mut self) {}
}
impl S {
    pub unsafe fn set(&mut self, mut x: i32) {
        self.v = x;
    }
}
impl S {
    pub unsafe fn add(&mut self, mut x: i32) -> i32 {
        self.v += x;
        return self.v;
    }
}
impl Derived {}
pub unsafe fn pair_sum_1(mut p: *const Pair) -> i32 {
    return (((*p).first) + ((*p).second));
}
unsafe impl Base for Derived {
    unsafe fn apply(&mut self, x: i32) -> i32 {
        return ((self.factor) * (x));
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
