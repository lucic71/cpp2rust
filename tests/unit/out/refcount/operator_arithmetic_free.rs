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
    pub fn move_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, v).write({ (*_a0.upgrade().deref()).v });
    }
}
pub fn operator_add_0(__ret: Ptr<S>, a: Ptr<S>, b: Ptr<S>) {
    {
        let __d = __ret.clone();
        field!(__d, v).write(({ a.with(|__s| __s.v) } + { b.with(|__s| __s.v) }));
    };
    return;
}
pub fn operator_sub_1(__ret: Ptr<S>, a: Ptr<S>, b: Ptr<S>) {
    {
        let __d = __ret.clone();
        field!(__d, v).write(({ a.with(|__s| __s.v) } - { b.with(|__s| __s.v) }));
    };
    return;
}
pub fn operator_mul_2(__ret: Ptr<S>, a: Ptr<S>, b: Ptr<S>) {
    {
        let __d = __ret.clone();
        field!(__d, v).write(({ a.with(|__s| __s.v) } * { b.with(|__s| __s.v) }));
    };
    return;
}
pub fn operator_div_3(__ret: Ptr<S>, a: Ptr<S>, b: Ptr<S>) {
    {
        let __d = __ret.clone();
        field!(__d, v).write(({ a.with(|__s| __s.v) } / { b.with(|__s| __s.v) }));
    };
    return;
}
pub fn operator_rem_4(__ret: Ptr<S>, a: Ptr<S>, b: Ptr<S>) {
    {
        let __d = __ret.clone();
        field!(__d, v).write(({ a.with(|__s| __s.v) } % { b.with(|__s| __s.v) }));
    };
    return;
}
pub fn operator_pos_5(__ret: Ptr<S>, a: Ptr<S>) {
    {
        let __d = __ret.clone();
        field!(__d, v).write(a.with(|__s| __s.v));
    };
    return;
}
pub fn operator_neg_6(__ret: Ptr<S>, a: Ptr<S>) {
    {
        let __d = __ret.clone();
        field!(__d, v).write(-a.with(|__s| __s.v));
    };
    return;
}
pub fn operator_inc_7(a: Ptr<S>) -> Ptr<S> {
    field!(a, v).with_mut(|__v| {
        __v.prefix_inc();
    });
    return (a).clone();
}
pub fn operator_post_inc_8(__ret: Ptr<S>, a: Ptr<S>, mut _a1: i32) {
    let old: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    S::copy_from(old.as_pointer(), { (a).clone() });
    field!(a, v).with_mut(|__v| {
        __v.prefix_inc();
    });
    S::move_from(__ret.clone(), { old.as_pointer() });
    return;
}
pub fn operator_dec_9(a: Ptr<S>) -> Ptr<S> {
    field!(a, v).with_mut(|__v| {
        __v.prefix_dec();
    });
    return (a).clone();
}
pub fn operator_post_dec_10(__ret: Ptr<S>, a: Ptr<S>, mut _a1: i32) {
    let old: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    S::copy_from(old.as_pointer(), { (a).clone() });
    field!(a, v).with_mut(|__v| {
        __v.prefix_dec();
    });
    S::move_from(__ret.clone(), { old.as_pointer() });
    return;
}
pub fn operator_add_11(__ret: Ptr<S>, a: Ptr<S>, mut b: i32) {
    {
        let __d = __ret.clone();
        field!(__d, v).write(({ a.with(|__s| __s.v) } + { b }));
    };
    return;
}
pub fn operator_add_12(__ret: Ptr<S>, mut a: i32, b: Ptr<S>) {
    {
        let __d = __ret.clone();
        field!(__d, v).write(({ a } + { b.with(|__s| __s.v) }));
    };
    return;
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = a.as_pointer();
        field!(__d, v).write(7);
    };
    let b: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = b.as_pointer();
        field!(__d, v).write(2);
    };
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({
                    let _a: Ptr<S> = a.as_pointer();
                    operator_add_0(__v.as_pointer(), _a, b.as_pointer())
                });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 9)
    );
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({
                    let _a: Ptr<S> = a.as_pointer();
                    operator_sub_1(__v.as_pointer(), _a, b.as_pointer())
                });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 5)
    );
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({
                    let _a: Ptr<S> = a.as_pointer();
                    operator_mul_2(__v.as_pointer(), _a, b.as_pointer())
                });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 14)
    );
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({
                    let _a: Ptr<S> = a.as_pointer();
                    operator_div_3(__v.as_pointer(), _a, b.as_pointer())
                });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 3)
    );
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({
                    let _a: Ptr<S> = a.as_pointer();
                    operator_rem_4(__v.as_pointer(), _a, b.as_pointer())
                });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 1)
    );
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({
                    let _a: Ptr<S> = a.as_pointer();
                    operator_pos_5(__v.as_pointer(), _a)
                });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 7)
    );
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({
                    let _a: Ptr<S> = a.as_pointer();
                    operator_neg_6(__v.as_pointer(), _a)
                });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == -7_i32)
    );
    assert!(
        (({
            let _a: Ptr<S> = a.as_pointer();
            operator_inc_7(_a)
        })
        .with(|__s| (__s).v)
            == 8)
    );
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({
                    let _a: Ptr<S> = a.as_pointer();
                    operator_post_inc_8(__v.as_pointer(), _a, 0)
                });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 8)
    );
    assert!(({ ({ (*a.borrow()).v } == 9) }));
    assert!(
        (({
            let _a: Ptr<S> = a.as_pointer();
            operator_dec_9(_a)
        })
        .with(|__s| (__s).v)
            == 8)
    );
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({
                    let _a: Ptr<S> = a.as_pointer();
                    operator_post_dec_10(__v.as_pointer(), _a, 0)
                });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 8)
    );
    assert!(({ ({ (*a.borrow()).v } == 7) }));
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({
                    let _a: Ptr<S> = a.as_pointer();
                    operator_add_11(__v.as_pointer(), _a, 1)
                });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 8)
    );
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({ operator_add_12(__v.as_pointer(), 1, a.as_pointer()) });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 8)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
