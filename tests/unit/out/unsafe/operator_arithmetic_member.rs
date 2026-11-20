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
    pub unsafe fn operator_add_1(&self, __ret: *mut S, o: *const S) {
        {
            let __d = __ret.clone();
            (&raw mut (*__d).v).write(((self.v) + ((*o).v)));
        };
        return;
    }
    pub unsafe fn operator_sub_2(&self, __ret: *mut S, o: *const S) {
        {
            let __d = __ret.clone();
            (&raw mut (*__d).v).write(((self.v) - ((*o).v)));
        };
        return;
    }
    pub unsafe fn operator_mul(&self, __ret: *mut S, o: *const S) {
        {
            let __d = __ret.clone();
            (&raw mut (*__d).v).write(((self.v) * ((*o).v)));
        };
        return;
    }
    pub unsafe fn operator_div(&self, __ret: *mut S, o: *const S) {
        {
            let __d = __ret.clone();
            (&raw mut (*__d).v).write(((self.v) / ((*o).v)));
        };
        return;
    }
    pub unsafe fn operator_rem(&self, __ret: *mut S, o: *const S) {
        {
            let __d = __ret.clone();
            (&raw mut (*__d).v).write(((self.v) % ((*o).v)));
        };
        return;
    }
    pub unsafe fn operator_pos_6(&self, __ret: *mut S) {
        {
            let __d = __ret.clone();
            (&raw mut (*__d).v).write(self.v);
        };
        return;
    }
    pub unsafe fn operator_neg_7(&self, __ret: *mut S) {
        {
            let __d = __ret.clone();
            (&raw mut (*__d).v).write(-self.v);
        };
        return;
    }
    pub unsafe fn operator_inc_8(&mut self) -> *mut S {
        self.v.prefix_inc();
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_post_inc_9(&mut self, __ret: *mut S, mut _a0: i32) {
        let mut old: S = ::std::mem::MaybeUninit::uninit().assume_init();
        S::copy_from(&raw mut old, { &(*(self as *mut S)) });
        self.v.prefix_inc();
        S::move_from(__ret.clone(), { &mut old });
        return;
    }
    pub unsafe fn operator_dec_10(&mut self) -> *mut S {
        self.v.prefix_dec();
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_post_dec_11(&mut self, __ret: *mut S, mut _a0: i32) {
        let mut old: S = ::std::mem::MaybeUninit::uninit().assume_init();
        S::copy_from(&raw mut old, { &(*(self as *mut S)) });
        self.v.prefix_dec();
        S::move_from(__ret.clone(), { &mut old });
        return;
    }
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
            (unsafe { S::operator_add_1(&a, &raw mut __v, &b) });
            __v
        })
        .v) == (9))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe { S::operator_sub_2(&a, &raw mut __v, &b) });
            __v
        })
        .v) == (5))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe { S::operator_mul(&a, &raw mut __v, &b) });
            __v
        })
        .v) == (14))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe { S::operator_div(&a, &raw mut __v, &b) });
            __v
        })
        .v) == (3))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe { S::operator_rem(&a, &raw mut __v, &b) });
            __v
        })
        .v) == (1))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe { S::operator_pos_6(&a, &raw mut __v) });
            __v
        })
        .v) == (7))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe { S::operator_neg_7(&a, &raw mut __v) });
            __v
        })
        .v) == (-7_i32))
    );
    assert!((((*(unsafe { S::operator_inc_8(&mut a,) })).v) == (8)));
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe { S::operator_post_inc_9(&mut a, &raw mut __v, 0) });
            __v
        })
        .v) == (8))
    );
    assert!(((a.v) == (9)));
    assert!((((*(unsafe { S::operator_dec_10(&mut a,) })).v) == (8)));
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe { S::operator_post_dec_11(&mut a, &raw mut __v, 0) });
            __v
        })
        .v) == (8))
    );
    assert!(((a.v) == (7)));
    assert!(
        (((*(unsafe { S::operator_inc_8(&mut (*(unsafe { S::operator_inc_8(&mut a,) })),) })).v)
            == (9))
    );
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe {
                let mut _o: S = ::std::mem::MaybeUninit::uninit().assume_init();
                {
                    let __d = &raw mut _o;
                    (&raw mut (*__d).v).write(4);
                };
                S::operator_add_1(
                    &mut *(|__p: *mut S| {
                        {
                            let __d = __p;
                            (&raw mut (*__d).v).write(3);
                        };
                        __p
                    })(
                        (&mut ::std::mem::MaybeUninit::<S>::uninit()).as_mut_ptr()
                    ),
                    &raw mut __v,
                    &mut _o,
                )
            });
            __v
        })
        .v) == (7))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
