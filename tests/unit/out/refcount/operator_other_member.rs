extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Static {}
impl Static {
    pub fn operator_call(mut a: i32, mut b: i32) -> i32 {
        return (a * b);
    }
    pub fn new_1(this: Ptr<Static>) {}
    pub fn copy_from(this: Ptr<Static>, _a0: Ptr<Static>) {}
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(i32, to_i32)]
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
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = s.as_pointer();
        field!(__d, v).write(3);
    };
    let t: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = t.as_pointer();
        field!(__d, v).write(4);
    };
    assert!((({ SImpl::operator_call_1(&s.as_pointer(),) }) == 3));
    assert!((({ SImpl::operator_call_2(&s.as_pointer(), 1,) }) == 4));
    assert!((({ SImpl::operator_call_3(&s.as_pointer(), 1, 2,) }) == 6));
    assert!(
        ({
            (Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                ({ SImpl::operator_comma(&s.as_pointer(), __v.as_pointer(), t.as_pointer()) });
                __v
            })
            .ok()
            .unwrap()
            .into_inner())
            .v
        } == 34)
    );
    let mut i: i32 = ({ SImpl::to_i32(&s.as_pointer()) });
    assert!((i == 3));
    assert!(((({ SImpl::to_i32(&s.as_pointer(),) }) + 1) == 4));
    if ({ SImpl::to_bool(&s.as_pointer()) }) {
        assert!(({ SImpl::to_bool(&s.as_pointer(),) }));
    } else {
        assert!(false);
    }
    let z: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = z.as_pointer();
        field!(__d, v).write(0);
    };
    assert!(({ SImpl::to_bool(&s.as_pointer(),) }));
    assert!(!({ SImpl::to_bool(&z.as_pointer(),) }));
    assert!(({ SImpl::to_bool(&s.as_pointer(),) }) && (!({ SImpl::to_bool(&z.as_pointer(),) })));
    let st: Value<Static> = Rc::new(RefCell::new(<Static>::uninit()));
    Static::new_1(st.as_pointer());
    assert!((({ Static::operator_call(6, 7,) }) == 42));
    assert!(
        (({
            SImpl::operator_call_1(
                &{
                    let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                    {
                        let __d = __v.as_pointer();
                        field!(__d, v).write(5);
                    };
                    __v
                }
                .as_pointer(),
            )
        }) == 5)
    );
    assert!(
        (({
            SImpl::operator_call_3(
                &{
                    let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                    {
                        let __d = __v.as_pointer();
                        field!(__d, v).write(5);
                    };
                    __v
                }
                .as_pointer(),
                1,
                1,
            )
        }) == 7)
    );
    return 0;
}
pub trait SImpl {
    fn operator_call_1(&self) -> i32;
    fn operator_call_2(&self, a: i32) -> i32;
    fn operator_call_3(&self, a: i32, b: i32) -> i32;
    fn operator_comma(&self, __ret: Ptr<S>, o: Ptr<S>);
    fn to_i32(&self) -> i32;
    fn to_bool(&self) -> bool;
}
impl SImpl for Ptr<S> {
    fn operator_call_1(&self) -> i32 {
        return (*self).with(|__s| __s.v);
    }
    fn operator_call_2(&self, mut a: i32) -> i32 {
        return ((*self).with(|__s| __s.v) + a);
    }
    fn operator_call_3(&self, mut a: i32, mut b: i32) -> i32 {
        return (((*self).with(|__s| __s.v) + a) + b);
    }
    fn operator_comma(&self, __ret: Ptr<S>, o: Ptr<S>) {
        {
            let __d = __ret.clone();
            field!(__d, v).write(({ ((*self).with(|__s| __s.v) * 10) } + { o.with(|__s| __s.v) }));
        };
        return;
    }
    fn to_i32(&self) -> i32 {
        return (*self).with(|__s| __s.v);
    }
    fn to_bool(&self) -> bool {
        return ((*self).with(|__s| __s.v) != 0);
    }
}
pub fn __cpp2rust_init_globals() {}
