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
    pub v: u32,
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
        field!(__d, v).write(12_u32);
    };
    let b: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = b.as_pointer();
        field!(__d, v).write(10_u32);
    };
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({ SImpl::operator_bitnot(&a.as_pointer(), __v.as_pointer()) });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == !12_u32)
    );
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({ SImpl::operator_bitand(&a.as_pointer(), __v.as_pointer(), b.as_pointer()) });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 8_u32)
    );
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({ SImpl::operator_bitor(&a.as_pointer(), __v.as_pointer(), b.as_pointer()) });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 14_u32)
    );
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({ SImpl::operator_bitxor(&a.as_pointer(), __v.as_pointer(), b.as_pointer()) });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 6_u32)
    );
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({ SImpl::operator_shl(&a.as_pointer(), __v.as_pointer(), 2) });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 48_u32)
    );
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({ SImpl::operator_shr(&a.as_pointer(), __v.as_pointer(), 2) });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 3_u32)
    );
    return 0;
}
pub trait SImpl {
    fn operator_bitnot(&self, __ret: Ptr<S>);
    fn operator_bitand(&self, __ret: Ptr<S>, o: Ptr<S>);
    fn operator_bitor(&self, __ret: Ptr<S>, o: Ptr<S>);
    fn operator_bitxor(&self, __ret: Ptr<S>, o: Ptr<S>);
    fn operator_shl(&self, __ret: Ptr<S>, n: i32);
    fn operator_shr(&self, __ret: Ptr<S>, n: i32);
}
impl SImpl for Ptr<S> {
    fn operator_bitnot(&self, __ret: Ptr<S>) {
        {
            let __d = __ret.clone();
            field!(__d, v).write(!(*self).with(|__s| __s.v));
        };
        return;
    }
    fn operator_bitand(&self, __ret: Ptr<S>, o: Ptr<S>) {
        {
            let __d = __ret.clone();
            field!(__d, v).write(({ (*self).with(|__s| __s.v) } & { o.with(|__s| __s.v) }));
        };
        return;
    }
    fn operator_bitor(&self, __ret: Ptr<S>, o: Ptr<S>) {
        {
            let __d = __ret.clone();
            field!(__d, v).write(({ (*self).with(|__s| __s.v) } | { o.with(|__s| __s.v) }));
        };
        return;
    }
    fn operator_bitxor(&self, __ret: Ptr<S>, o: Ptr<S>) {
        {
            let __d = __ret.clone();
            field!(__d, v).write(({ (*self).with(|__s| __s.v) } ^ { o.with(|__s| __s.v) }));
        };
        return;
    }
    fn operator_shl(&self, __ret: Ptr<S>, mut n: i32) {
        {
            let __d = __ret.clone();
            field!(__d, v).write(((*self).with(|__s| __s.v) << n));
        };
        return;
    }
    fn operator_shr(&self, __ret: Ptr<S>, mut n: i32) {
        {
            let __d = __ret.clone();
            field!(__d, v).write(((*self).with(|__s| __s.v) >> n));
        };
        return;
    }
}
pub fn __cpp2rust_init_globals() {}
