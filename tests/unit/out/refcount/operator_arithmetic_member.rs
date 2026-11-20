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
    pub fn new_1(this: Ptr<S>) {}
    pub fn copy_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
    pub fn move_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, v).write({ (*_a0.upgrade().deref()).v });
    }
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
                ({ SImpl::operator_add_1(&a.as_pointer(), __v.as_pointer(), b.as_pointer()) });
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
                ({ SImpl::operator_sub_2(&a.as_pointer(), __v.as_pointer(), b.as_pointer()) });
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
                ({ SImpl::operator_mul(&a.as_pointer(), __v.as_pointer(), b.as_pointer()) });
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
                ({ SImpl::operator_div(&a.as_pointer(), __v.as_pointer(), b.as_pointer()) });
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
                ({ SImpl::operator_rem(&a.as_pointer(), __v.as_pointer(), b.as_pointer()) });
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
                ({ SImpl::operator_pos_6(&a.as_pointer(), __v.as_pointer()) });
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
                ({ SImpl::operator_neg_7(&a.as_pointer(), __v.as_pointer()) });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == -7_i32)
    );
    assert!((({ SImpl::operator_inc_8(&a.as_pointer(),) }).with(|__s| (__s).v) == 8));
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({ SImpl::operator_post_inc_9(&a.as_pointer(), __v.as_pointer(), 0) });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 8)
    );
    assert!(({ ({ (*a.borrow()).v } == 9) }));
    assert!((({ SImpl::operator_dec_10(&a.as_pointer(),) }).with(|__s| (__s).v) == 8));
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({ SImpl::operator_post_dec_11(&a.as_pointer(), __v.as_pointer(), 0) });
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
        (({ SImpl::operator_inc_8(&({ SImpl::operator_inc_8(&a.as_pointer(),) }),) })
            .with(|__s| (__s).v)
            == 9)
    );
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({
                    let _o: Value<S> = {
                        let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                        {
                            let __d = __v.as_pointer();
                            field!(__d, v).write(4);
                        };
                        __v
                    };
                    SImpl::operator_add_1(
                        &{
                            let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                            {
                                let __d = __v.as_pointer();
                                field!(__d, v).write(3);
                            };
                            __v
                        }
                        .as_pointer(),
                        __v.as_pointer(),
                        _o.as_pointer(),
                    )
                });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 7)
    );
    return 0;
}
pub trait SImpl {
    fn operator_add_1(&self, __ret: Ptr<S>, o: Ptr<S>);
    fn operator_sub_2(&self, __ret: Ptr<S>, o: Ptr<S>);
    fn operator_mul(&self, __ret: Ptr<S>, o: Ptr<S>);
    fn operator_div(&self, __ret: Ptr<S>, o: Ptr<S>);
    fn operator_rem(&self, __ret: Ptr<S>, o: Ptr<S>);
    fn operator_pos_6(&self, __ret: Ptr<S>);
    fn operator_neg_7(&self, __ret: Ptr<S>);
    fn operator_inc_8(&self) -> Ptr<S>;
    fn operator_post_inc_9(&self, __ret: Ptr<S>, _a0: i32);
    fn operator_dec_10(&self) -> Ptr<S>;
    fn operator_post_dec_11(&self, __ret: Ptr<S>, _a0: i32);
}
impl SImpl for Ptr<S> {
    fn operator_add_1(&self, __ret: Ptr<S>, o: Ptr<S>) {
        {
            let __d = __ret.clone();
            field!(__d, v).write(({ (*self).with(|__s| __s.v) } + { o.with(|__s| __s.v) }));
        };
        return;
    }
    fn operator_sub_2(&self, __ret: Ptr<S>, o: Ptr<S>) {
        {
            let __d = __ret.clone();
            field!(__d, v).write(({ (*self).with(|__s| __s.v) } - { o.with(|__s| __s.v) }));
        };
        return;
    }
    fn operator_mul(&self, __ret: Ptr<S>, o: Ptr<S>) {
        {
            let __d = __ret.clone();
            field!(__d, v).write(({ (*self).with(|__s| __s.v) } * { o.with(|__s| __s.v) }));
        };
        return;
    }
    fn operator_div(&self, __ret: Ptr<S>, o: Ptr<S>) {
        {
            let __d = __ret.clone();
            field!(__d, v).write(({ (*self).with(|__s| __s.v) } / { o.with(|__s| __s.v) }));
        };
        return;
    }
    fn operator_rem(&self, __ret: Ptr<S>, o: Ptr<S>) {
        {
            let __d = __ret.clone();
            field!(__d, v).write(({ (*self).with(|__s| __s.v) } % { o.with(|__s| __s.v) }));
        };
        return;
    }
    fn operator_pos_6(&self, __ret: Ptr<S>) {
        {
            let __d = __ret.clone();
            field!(__d, v).write((*self).with(|__s| __s.v));
        };
        return;
    }
    fn operator_neg_7(&self, __ret: Ptr<S>) {
        {
            let __d = __ret.clone();
            field!(__d, v).write(-(*self).with(|__s| __s.v));
        };
        return;
    }
    fn operator_inc_8(&self) -> Ptr<S> {
        field!((*self), v).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_post_inc_9(&self, __ret: Ptr<S>, mut _a0: i32) {
        let old: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
        S::copy_from(old.as_pointer(), { (*self).clone() });
        field!((*self), v).with_mut(|__v| {
            __v.prefix_inc();
        });
        S::move_from(__ret.clone(), { old.as_pointer() });
        return;
    }
    fn operator_dec_10(&self) -> Ptr<S> {
        field!((*self), v).with_mut(|__v| {
            __v.prefix_dec();
        });
        return (*self).clone();
    }
    fn operator_post_dec_11(&self, __ret: Ptr<S>, mut _a0: i32) {
        let old: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
        S::copy_from(old.as_pointer(), { (*self).clone() });
        field!((*self), v).with_mut(|__v| {
            __v.prefix_dec();
        });
        S::move_from(__ret.clone(), { old.as_pointer() });
        return;
    }
}
pub fn __cpp2rust_init_globals() {}
