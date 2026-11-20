extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub v: i32,
}
impl S {
    pub fn copy_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
}
pub fn operator_div_0(a: Ptr<S>, b: Ptr<S>) -> i32 {
    return ({ a.with(|__s| __s.v) } / { b.with(|__s| __s.v) });
}
pub fn operator_div_1(a: Value<S>, mut b: i32) -> i32 {
    return (({ (*a.borrow()).v } / b) + 1);
}
pub fn operator_rem_2(a: Value<S>, b: Value<S>) -> i32 {
    return ({ (*a.borrow()).v } % { (*b.borrow()).v });
}
pub fn operator_rem_3(a: Ptr<S>, mut b: i32) -> i32 {
    return (({ a.with(|__s| __s.v) } % { b }) + 1);
}
pub fn operator_eq_4(mut a: i32, b: Value<S>) -> i32 {
    return if (a == { (*b.borrow()).v }) { 4 } else { 0 };
}
pub fn operator_eq_5(mut a: i64, b: Ptr<S>) -> i32 {
    return if ({ a } == { (b.with(|__s| __s.v) as i64) }) {
        5
    } else {
        0
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = s.as_pointer();
        field!(__d, v).write(6);
    };
    let t: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = t.as_pointer();
        field!(__d, v).write(4);
    };
    assert!((({ SImpl::operator_eq_1(&s.as_pointer(), 6,) }) == 1));
    assert!((({ SImpl::operator_eq_2(&s.as_pointer(), 6_i64,) }) == 2));
    assert!((({ SImpl::operator_eq_3(&s.as_pointer(), 6_f64,) }) == 3));
    assert!((({ SImpl::operator_eq_1(&s.as_pointer(), 7,) }) == 0));
    assert!((({ SImpl::operator_add(&s.as_pointer(), t.as_pointer(),) }) == 10));
    assert!(
        (({
            SImpl::operator_sub(&s.as_pointer(), {
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                S::copy_from(__v.as_pointer(), { t.as_pointer() });
                __v
            })
        }) == 2)
    );
    assert!((({ SImpl::operator_mul_6(&s.as_pointer(), t.as_pointer(),) }) == 24));
    assert!((({ SImpl::operator_mul_7(&s.as_pointer(), 2,) }) == 13));
    assert!(
        (({
            let _a: Ptr<S> = s.as_pointer();
            operator_div_0(_a, t.as_pointer())
        }) == 1)
    );
    assert!(
        (({
            let _a: S = {
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                S::copy_from(__v.as_pointer(), { s.as_pointer() });
                __v
            };
            operator_div_1(_a, 4)
        }) == 2)
    );
    assert!(
        (({
            let _a: S = {
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                S::copy_from(__v.as_pointer(), { s.as_pointer() });
                __v
            };
            operator_rem_2(_a, {
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                S::copy_from(__v.as_pointer(), { t.as_pointer() });
                __v
            })
        }) == 2)
    );
    assert!(
        (({
            let _a: Ptr<S> = s.as_pointer();
            operator_rem_3(_a, 4)
        }) == 3)
    );
    assert!(
        (({
            operator_eq_4(6, {
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                S::copy_from(__v.as_pointer(), { s.as_pointer() });
                __v
            })
        }) == 4)
    );
    assert!((({ operator_eq_5(6_i64, s.as_pointer(),) }) == 5));
    return 0;
}
pub trait SImpl {
    fn operator_eq_1(&self, o: i32) -> i32;
    fn operator_eq_2(&self, o: i64) -> i32;
    fn operator_eq_3(&self, o: f64) -> i32;
    fn operator_add(&self, o: Ptr<S>) -> i32;
    fn operator_sub(&self, o: Value<S>) -> i32;
    fn operator_mul_6(&self, o: Ptr<S>) -> i32;
    fn operator_mul_7(&self, o: i32) -> i32;
}
impl SImpl for Ptr<S> {
    fn operator_eq_1(&self, mut o: i32) -> i32 {
        return if ((*self).with(|__s| __s.v) == o) {
            1
        } else {
            0
        };
    }
    fn operator_eq_2(&self, mut o: i64) -> i32 {
        return if (((*self).with(|__s| __s.v) as i64) == o) {
            2
        } else {
            0
        };
    }
    fn operator_eq_3(&self, mut o: f64) -> i32 {
        return if (((*self).with(|__s| __s.v) as f64) == o) {
            3
        } else {
            0
        };
    }
    fn operator_add(&self, o: Ptr<S>) -> i32 {
        return ({ (*self).with(|__s| __s.v) } + { o.with(|__s| __s.v) });
    }
    fn operator_sub(&self, o: Value<S>) -> i32 {
        return ((*self).with(|__s| __s.v) - { (*o.borrow()).v });
    }
    fn operator_mul_6(&self, o: Ptr<S>) -> i32 {
        return ({ (*self).with(|__s| __s.v) } * { o.with(|__s| __s.v) });
    }
    fn operator_mul_7(&self, mut o: i32) -> i32 {
        return (((*self).with(|__s| __s.v) * o) + 1);
    }
}
pub fn __cpp2rust_init_globals() {}
