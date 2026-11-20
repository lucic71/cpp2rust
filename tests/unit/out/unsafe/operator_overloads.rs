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
    pub unsafe fn operator_eq_1(&self, mut o: i32) -> i32 {
        return if ((self.v) == (o)) { 1 } else { 0 };
    }
    pub unsafe fn operator_eq_2(&self, mut o: i64) -> i32 {
        return if ((self.v as i64) == (o)) { 2 } else { 0 };
    }
    pub unsafe fn operator_eq_3(&self, mut o: f64) -> i32 {
        return if ((self.v as f64) == (o)) { 3 } else { 0 };
    }
    pub unsafe fn operator_add(&self, o: *const S) -> i32 {
        return ((self.v) + ((*o).v));
    }
    pub unsafe fn operator_sub(&self, o: *mut S) -> i32 {
        return ((self.v) - ((*o).v));
    }
    pub unsafe fn operator_mul_6(&self, o: *const S) -> i32 {
        return ((self.v) * ((*o).v));
    }
    pub unsafe fn operator_mul_7(&self, mut o: i32) -> i32 {
        return (((self.v) * (o)) + (1));
    }
    pub unsafe fn new_1(this: *mut S) {}
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {
        (&raw mut (*this).v).write((*_a0).v);
    }
    pub unsafe fn move_from(this: *mut S, _a0: *mut S) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
pub unsafe fn operator_div_0(a: *const S, b: *const S) -> i32 {
    return (((*a).v) / ((*b).v));
}
pub unsafe fn operator_div_1(a: *mut S, mut b: i32) -> i32 {
    return ((((*a).v) / (b)) + (1));
}
pub unsafe fn operator_rem_2(a: *mut S, b: *mut S) -> i32 {
    return (((*a).v) % ((*b).v));
}
pub unsafe fn operator_rem_3(a: *const S, mut b: i32) -> i32 {
    return ((((*a).v) % (b)) + (1));
}
pub unsafe fn operator_eq_4(mut a: i32, b: *mut S) -> i32 {
    return if ((a) == ((*b).v)) { 4 } else { 0 };
}
pub unsafe fn operator_eq_5(mut a: i64, b: *const S) -> i32 {
    return if ((a) == ((*b).v as i64)) { 5 } else { 0 };
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
        (&raw mut (*__d).v).write(6);
    };
    let mut t: S = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut t;
        (&raw mut (*__d).v).write(4);
    };
    assert!(((unsafe { S::operator_eq_1(&s, 6,) }) == (1)));
    assert!(((unsafe { S::operator_eq_2(&s, 6_i64,) }) == (2)));
    assert!(((unsafe { S::operator_eq_3(&s, 6_f64,) }) == (3)));
    assert!(((unsafe { S::operator_eq_1(&s, 7,) }) == (0)));
    assert!(((unsafe { S::operator_add(&s, &t,) }) == (10)));
    assert!(
        ((unsafe {
            S::operator_sub(
                &s,
                (|__p: *mut S| {
                    S::copy_from(__p, { &t });
                    __p
                })((&mut ::std::mem::MaybeUninit::<S>::uninit()).as_mut_ptr()),
            )
        }) == (2))
    );
    assert!(((unsafe { S::operator_mul_6(&s, &t,) }) == (24)));
    assert!(((unsafe { S::operator_mul_7(&s, 2,) }) == (13)));
    assert!(
        ((unsafe {
            let _a: *const S = &s;
            operator_div_0(_a, &t)
        }) == (1))
    );
    assert!(
        ((unsafe {
            let mut __tmp_0: S = ::std::mem::MaybeUninit::uninit().assume_init();
            S::copy_from(&raw mut __tmp_0, { &s });
            let _a = &raw mut __tmp_0;
            operator_div_1(_a, 4)
        }) == (2))
    );
    assert!(
        ((unsafe {
            let mut __tmp_1: S = ::std::mem::MaybeUninit::uninit().assume_init();
            S::copy_from(&raw mut __tmp_1, { &s });
            let _a = &raw mut __tmp_1;
            operator_rem_2(
                _a,
                (|__p: *mut S| {
                    S::copy_from(__p, { &t });
                    __p
                })((&mut ::std::mem::MaybeUninit::<S>::uninit()).as_mut_ptr()),
            )
        }) == (2))
    );
    assert!(
        ((unsafe {
            let _a: *const S = &s;
            operator_rem_3(_a, 4)
        }) == (3))
    );
    assert!(
        ((unsafe {
            operator_eq_4(
                6,
                (|__p: *mut S| {
                    S::copy_from(__p, { &s });
                    __p
                })((&mut ::std::mem::MaybeUninit::<S>::uninit()).as_mut_ptr()),
            )
        }) == (4))
    );
    assert!(((unsafe { operator_eq_5(6_i64, &s,) }) == (5)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
