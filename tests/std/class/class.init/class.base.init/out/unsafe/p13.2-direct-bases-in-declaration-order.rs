extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut sequence_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe fn stampArgument_1(stamp: *mut i32, mut value: i32) -> i32 {
    (*stamp) = (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)).prefix_inc();
    return value;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct FirstBase {
    pub value: i32,
    pub stamp: i32,
}
impl FirstBase {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self {
            value: v,
            stamp: (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)).prefix_inc(),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct SecondBase {
    pub value: i32,
    pub stamp: i32,
}
impl SecondBase {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self {
            value: v,
            stamp: (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)).prefix_inc(),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct InitializesBasesOutOfOrder {
    pub firstArgumentStamp: i32,
    pub secondArgumentStamp: i32,
}
impl InitializesBasesOutOfOrder {
    pub unsafe fn new(mut a: i32) -> Self {
        let mut this = Self {
            firstArgumentStamp: 0_i32,
            secondArgumentStamp: 0_i32,
        };
        this
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut d: InitializesBasesOutOfOrder = InitializesBasesOutOfOrder::new({ 10 });
    assert!((((d as FirstBase).value) == (12)));
    assert!((((d as SecondBase).value) == (11)));
    assert!(((d.firstArgumentStamp) == (1)));
    assert!((((d as FirstBase).stamp) == (2)));
    assert!(((d.secondArgumentStamp) == (3)));
    assert!((((d as SecondBase).stamp) == (4)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)) == (4)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const sequence_0);
}
