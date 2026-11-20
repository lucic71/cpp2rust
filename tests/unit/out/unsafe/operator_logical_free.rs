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
pub struct S {
    pub v: i32,
}
impl S {
    pub unsafe fn new_1(this: *mut S) {}
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {
        (&raw mut (*this).v).write((*_a0).v);
    }
    pub unsafe fn move_from(this: *mut S, _a0: *mut S) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
pub unsafe fn operator_not_0(a: *const S) -> bool {
    return (((*a).v) == (0));
}
pub unsafe fn operator_and_1(a: *const S, b: *const S) -> bool {
    return (((*a).v) != (0)) && (((*b).v) != (0));
}
pub unsafe fn operator_or_2(a: *const S, b: *const S) -> bool {
    return (((*a).v) != (0)) || (((*b).v) != (0));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut t: S = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut t;
        (&raw mut (*__d).v).write(1);
    };
    let mut f: S = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut f;
        (&raw mut (*__d).v).write(0);
    };
    assert!(
        (unsafe {
            let _a: *const S = &f;
            operator_not_0(_a)
        })
    );
    assert!(
        !(unsafe {
            let _a: *const S = &t;
            operator_not_0(_a)
        })
    );
    assert!(
        (unsafe {
            let _a: *const S = &t;
            let _b: *const S = &t;
            operator_and_1(_a, _b)
        })
    );
    assert!(
        !(unsafe {
            let _a: *const S = &t;
            operator_and_1(_a, &f)
        })
    );
    assert!(
        (unsafe {
            let _a: *const S = &t;
            operator_or_2(_a, &f)
        })
    );
    assert!(
        !(unsafe {
            let _a: *const S = &f;
            let _b: *const S = &f;
            operator_or_2(_a, _b)
        })
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
