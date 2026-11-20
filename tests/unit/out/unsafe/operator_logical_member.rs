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
    pub unsafe fn operator_not(&self) -> bool {
        return ((self.v) == (0));
    }
    pub unsafe fn operator_and(&self, o: *const S) -> bool {
        return ((self.v) != (0)) && (((*o).v) != (0));
    }
    pub unsafe fn operator_or(&self, o: *const S) -> bool {
        return ((self.v) != (0)) || (((*o).v) != (0));
    }
    pub unsafe fn new_1(this: *mut S) {}
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {
        (&raw mut (*this).v).write((*_a0).v);
    }
    pub unsafe fn move_from(this: *mut S, _a0: *mut S) {
        (&raw mut (*this).v).write((*_a0).v);
    }
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
    assert!((unsafe { S::operator_not(&f,) }));
    assert!(!(unsafe { S::operator_not(&t,) }));
    assert!(
        (unsafe {
            let _o: *const S = &t;
            S::operator_and(&t, _o)
        })
    );
    assert!(!(unsafe { S::operator_and(&t, &f,) }));
    assert!((unsafe { S::operator_or(&t, &f,) }));
    assert!(
        !(unsafe {
            let _o: *const S = &f;
            S::operator_or(&f, _o)
        })
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
