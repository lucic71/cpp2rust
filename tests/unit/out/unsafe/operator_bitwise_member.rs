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
    pub v: u32,
}
impl S {
    pub unsafe fn operator_bitnot(&self, __ret: *mut S) {
        {
            let __d = __ret.clone();
            (&raw mut (*__d).v).write(!self.v);
        };
        return;
    }
    pub unsafe fn operator_bitand(&self, __ret: *mut S, o: *const S) {
        {
            let __d = __ret.clone();
            (&raw mut (*__d).v).write(((self.v) & ((*o).v)));
        };
        return;
    }
    pub unsafe fn operator_bitor(&self, __ret: *mut S, o: *const S) {
        {
            let __d = __ret.clone();
            (&raw mut (*__d).v).write(((self.v) | ((*o).v)));
        };
        return;
    }
    pub unsafe fn operator_bitxor(&self, __ret: *mut S, o: *const S) {
        {
            let __d = __ret.clone();
            (&raw mut (*__d).v).write(((self.v) ^ ((*o).v)));
        };
        return;
    }
    pub unsafe fn operator_shl(&self, __ret: *mut S, mut n: i32) {
        {
            let __d = __ret.clone();
            (&raw mut (*__d).v).write(((self.v) << (n)));
        };
        return;
    }
    pub unsafe fn operator_shr(&self, __ret: *mut S, mut n: i32) {
        {
            let __d = __ret.clone();
            (&raw mut (*__d).v).write(((self.v) >> (n)));
        };
        return;
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
    let mut a: S = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut a;
        (&raw mut (*__d).v).write(12_u32);
    };
    let mut b: S = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut b;
        (&raw mut (*__d).v).write(10_u32);
    };
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe { S::operator_bitnot(&a, &raw mut __v) });
            __v
        })
        .v) == (!12_u32))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe { S::operator_bitand(&a, &raw mut __v, &b) });
            __v
        })
        .v) == (8_u32))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe { S::operator_bitor(&a, &raw mut __v, &b) });
            __v
        })
        .v) == (14_u32))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe { S::operator_bitxor(&a, &raw mut __v, &b) });
            __v
        })
        .v) == (6_u32))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe { S::operator_shl(&a, &raw mut __v, 2) });
            __v
        })
        .v) == (48_u32))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe { S::operator_shr(&a, &raw mut __v, 2) });
            __v
        })
        .v) == (3_u32))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
