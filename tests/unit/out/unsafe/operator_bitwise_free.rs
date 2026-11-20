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
    pub unsafe fn new_1(this: *mut S) {}
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {
        (&raw mut (*this).v).write((*_a0).v);
    }
    pub unsafe fn move_from(this: *mut S, _a0: *mut S) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
pub unsafe fn operator_bitnot_0(__ret: *mut S, a: *const S) {
    {
        let __d = __ret.clone();
        (&raw mut (*__d).v).write(!(*a).v);
    };
    return;
}
pub unsafe fn operator_bitand_1(__ret: *mut S, a: *const S, b: *const S) {
    {
        let __d = __ret.clone();
        (&raw mut (*__d).v).write((((*a).v) & ((*b).v)));
    };
    return;
}
pub unsafe fn operator_bitor_2(__ret: *mut S, a: *const S, b: *const S) {
    {
        let __d = __ret.clone();
        (&raw mut (*__d).v).write((((*a).v) | ((*b).v)));
    };
    return;
}
pub unsafe fn operator_bitxor_3(__ret: *mut S, a: *const S, b: *const S) {
    {
        let __d = __ret.clone();
        (&raw mut (*__d).v).write((((*a).v) ^ ((*b).v)));
    };
    return;
}
pub unsafe fn operator_shl_4(__ret: *mut S, a: *const S, mut n: i32) {
    {
        let __d = __ret.clone();
        (&raw mut (*__d).v).write((((*a).v) << (n)));
    };
    return;
}
pub unsafe fn operator_shr_5(__ret: *mut S, a: *const S, mut n: i32) {
    {
        let __d = __ret.clone();
        (&raw mut (*__d).v).write((((*a).v) >> (n)));
    };
    return;
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
            (unsafe {
                let _a: *const S = &a;
                operator_bitnot_0(&raw mut __v, _a)
            });
            __v
        })
        .v) == (!12_u32))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe {
                let _a: *const S = &a;
                operator_bitand_1(&raw mut __v, _a, &b)
            });
            __v
        })
        .v) == (8_u32))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe {
                let _a: *const S = &a;
                operator_bitor_2(&raw mut __v, _a, &b)
            });
            __v
        })
        .v) == (14_u32))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe {
                let _a: *const S = &a;
                operator_bitxor_3(&raw mut __v, _a, &b)
            });
            __v
        })
        .v) == (6_u32))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe {
                let _a: *const S = &a;
                operator_shl_4(&raw mut __v, _a, 2)
            });
            __v
        })
        .v) == (48_u32))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe {
                let _a: *const S = &a;
                operator_shr_5(&raw mut __v, _a, 2)
            });
            __v
        })
        .v) == (3_u32))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
