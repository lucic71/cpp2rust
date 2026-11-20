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
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {
        (&raw mut (*this).v).write((*_a0).v);
    }
    pub unsafe fn move_from(this: *mut S, _a0: *mut S) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
pub unsafe fn operator_add_0(__ret: *mut S, a: *const S, b: *const S) {
    {
        let __d = __ret.clone();
        (&raw mut (*__d).v).write((((*a).v) + ((*b).v)));
    };
    return;
}
pub unsafe fn operator_sub_1(__ret: *mut S, a: *const S, b: *const S) {
    {
        let __d = __ret.clone();
        (&raw mut (*__d).v).write((((*a).v) - ((*b).v)));
    };
    return;
}
pub unsafe fn operator_mul_2(__ret: *mut S, a: *const S, b: *const S) {
    {
        let __d = __ret.clone();
        (&raw mut (*__d).v).write((((*a).v) * ((*b).v)));
    };
    return;
}
pub unsafe fn operator_div_3(__ret: *mut S, a: *const S, b: *const S) {
    {
        let __d = __ret.clone();
        (&raw mut (*__d).v).write((((*a).v) / ((*b).v)));
    };
    return;
}
pub unsafe fn operator_rem_4(__ret: *mut S, a: *const S, b: *const S) {
    {
        let __d = __ret.clone();
        (&raw mut (*__d).v).write((((*a).v) % ((*b).v)));
    };
    return;
}
pub unsafe fn operator_pos_5(__ret: *mut S, a: *const S) {
    {
        let __d = __ret.clone();
        (&raw mut (*__d).v).write((*a).v);
    };
    return;
}
pub unsafe fn operator_neg_6(__ret: *mut S, a: *const S) {
    {
        let __d = __ret.clone();
        (&raw mut (*__d).v).write(-(*a).v);
    };
    return;
}
pub unsafe fn operator_inc_7(a: *mut S) -> *mut S {
    (*a).v.prefix_inc();
    return a;
}
pub unsafe fn operator_post_inc_8(__ret: *mut S, a: *mut S, mut _a1: i32) {
    let mut old: S = ::std::mem::MaybeUninit::uninit().assume_init();
    S::copy_from(&raw mut old, { a });
    (*a).v.prefix_inc();
    S::move_from(__ret.clone(), { &mut old });
    return;
}
pub unsafe fn operator_dec_9(a: *mut S) -> *mut S {
    (*a).v.prefix_dec();
    return a;
}
pub unsafe fn operator_post_dec_10(__ret: *mut S, a: *mut S, mut _a1: i32) {
    let mut old: S = ::std::mem::MaybeUninit::uninit().assume_init();
    S::copy_from(&raw mut old, { a });
    (*a).v.prefix_dec();
    S::move_from(__ret.clone(), { &mut old });
    return;
}
pub unsafe fn operator_add_11(__ret: *mut S, a: *const S, mut b: i32) {
    {
        let __d = __ret.clone();
        (&raw mut (*__d).v).write((((*a).v) + (b)));
    };
    return;
}
pub unsafe fn operator_add_12(__ret: *mut S, mut a: i32, b: *const S) {
    {
        let __d = __ret.clone();
        (&raw mut (*__d).v).write(((a) + ((*b).v)));
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
        (&raw mut (*__d).v).write(7);
    };
    let mut b: S = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut b;
        (&raw mut (*__d).v).write(2);
    };
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe {
                let _a: *const S = &a;
                operator_add_0(&raw mut __v, _a, &b)
            });
            __v
        })
        .v) == (9))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe {
                let _a: *const S = &a;
                operator_sub_1(&raw mut __v, _a, &b)
            });
            __v
        })
        .v) == (5))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe {
                let _a: *const S = &a;
                operator_mul_2(&raw mut __v, _a, &b)
            });
            __v
        })
        .v) == (14))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe {
                let _a: *const S = &a;
                operator_div_3(&raw mut __v, _a, &b)
            });
            __v
        })
        .v) == (3))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe {
                let _a: *const S = &a;
                operator_rem_4(&raw mut __v, _a, &b)
            });
            __v
        })
        .v) == (1))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe {
                let _a: *const S = &a;
                operator_pos_5(&raw mut __v, _a)
            });
            __v
        })
        .v) == (7))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe {
                let _a: *const S = &a;
                operator_neg_6(&raw mut __v, _a)
            });
            __v
        })
        .v) == (-7_i32))
    );
    assert!(
        (((*(unsafe {
            let _a: *mut S = &mut a;
            operator_inc_7(_a)
        }))
        .v) == (8))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe {
                let _a: *mut S = &mut a;
                operator_post_inc_8(&raw mut __v, _a, 0)
            });
            __v
        })
        .v) == (8))
    );
    assert!(((a.v) == (9)));
    assert!(
        (((*(unsafe {
            let _a: *mut S = &mut a;
            operator_dec_9(_a)
        }))
        .v) == (8))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe {
                let _a: *mut S = &mut a;
                operator_post_dec_10(&raw mut __v, _a, 0)
            });
            __v
        })
        .v) == (8))
    );
    assert!(((a.v) == (7)));
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe {
                let _a: *const S = &a;
                operator_add_11(&raw mut __v, _a, 1)
            });
            __v
        })
        .v) == (8))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe { operator_add_12(&raw mut __v, 1, &a) });
            __v
        })
        .v) == (8))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
