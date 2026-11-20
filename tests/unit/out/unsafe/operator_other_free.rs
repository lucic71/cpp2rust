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
}
pub unsafe fn operator_comma_0(__ret: *mut S, a: *const S, b: *const S) {
    {
        let __d = __ret.clone();
        (&raw mut (*__d).v).write(((((*a).v) * (10)) + ((*b).v)));
    };
    return;
}
pub unsafe fn operator_literal__k_1(mut v: u64) -> i64 {
    return (((v).wrapping_mul(1000_u64)) as i64);
}
pub unsafe fn operator_literal__half_2(mut v: f64) -> f64 {
    return ((v) / (2_f64));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut s;
        (&raw mut (*__d).v).write(3);
    };
    let mut t: S = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut t;
        (&raw mut (*__d).v).write(4);
    };
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe {
                let _a: *const S = &s;
                operator_comma_0(&raw mut __v, _a, &t)
            });
            __v
        })
        .v) == (34))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe {
                let mut _a: S = ::std::mem::MaybeUninit::uninit().assume_init();
                (unsafe {
                    let _a: *const S = &s;
                    operator_comma_0(&raw mut _a, _a, &t)
                });
                let _b: *const S = &s;
                operator_comma_0(&raw mut __v, &mut _a, _b)
            });
            __v
        })
        .v) == (343))
    );
    assert!(((unsafe { operator_literal__k_1(2_u64,) }) == (2000_i64)));
    assert!(((unsafe { operator_literal__half_2(3_f64,) }) == (1.5_f64)));
    assert!(((unsafe { operator_literal__k_1(4_u64,) }) == (4000_i64)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
