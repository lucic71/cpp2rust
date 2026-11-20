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
pub struct Pair {
    pub first: i32,
    pub second: i32,
}
impl Pair {
    pub unsafe fn copy_from(this: *mut Pair, _a0: *const Pair) {
        (&raw mut (*this).first).write((*_a0).first);
        (&raw mut (*this).second).write((*_a0).second);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut __decomp_0: Pair = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut __decomp_0;
        (&raw mut (*__d).first).write(1);
        (&raw mut (*__d).second).write(2);
    };
    assert!(((__decomp_0.first) == (1)));
    assert!(((__decomp_0.second) == (2)));
    let mut p: Pair = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut p;
        (&raw mut (*__d).first).write(10);
        (&raw mut (*__d).second).write(20);
    };
    let mut __decomp_1: Pair = ::std::mem::MaybeUninit::uninit().assume_init();
    Pair::copy_from(&raw mut __decomp_1, { &p });
    __decomp_1.first = 11;
    __decomp_1.second += 1;
    assert!(((__decomp_1.first) == (11)));
    assert!(((__decomp_1.second) == (21)));
    assert!(((p.first) == (10)));
    assert!(((p.second) == (20)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
