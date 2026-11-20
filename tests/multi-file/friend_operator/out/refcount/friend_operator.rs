extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct S {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
impl S {
    pub fn new_1(this: Ptr<S>) {}
    pub fn copy_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, a).write(_a0.with(|__s| __s.a));
        field!(this, b).write(_a0.with(|__s| __s.b));
    }
    pub fn move_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, a).write({ (*_a0.upgrade().deref()).a });
        field!(this, b).write({ (*_a0.upgrade().deref()).b });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let s1: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = s1.as_pointer();
        field!(__d, a).write(1);
        field!(__d, b).write(2);
    };
    let s2: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = s2.as_pointer();
        field!(__d, a).write(1);
        field!(__d, b).write(2);
    };
    let s3: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = s3.as_pointer();
        field!(__d, a).write(1);
        field!(__d, b).write(3);
    };
    assert!(
        ({
            let _x: Ptr<S> = s1.as_pointer();
            operator_eq_0(_x, s2.as_pointer())
        })
    );
    assert!(
        ({
            let _x: Ptr<S> = s1.as_pointer();
            operator_ne_1(_x, s3.as_pointer())
        })
    );
    assert!(
        !({
            let _x: Ptr<S> = s1.as_pointer();
            operator_eq_0(_x, s3.as_pointer())
        })
    );
    assert!(
        ({
            let _x: Ptr<S> = s1.as_pointer();
            operator_lt_2(_x, s3.as_pointer())
        })
    );
    assert!(
        !({
            let _x: Ptr<S> = s3.as_pointer();
            operator_lt_2(_x, s1.as_pointer())
        })
    );
    assert!((({ compare_3(s1.as_pointer(), s3.as_pointer(),) }) == -1_i32));
    assert!((({ compare_3(s3.as_pointer(), s1.as_pointer(),) }) == 1));
    assert!((({ compare_3(s1.as_pointer(), s2.as_pointer(),) }) == 0));
    return 0;
}
pub fn operator_eq_0(x: Ptr<S>, y: Ptr<S>) -> bool {
    return ({ x.with(|__s| __s.a) } == { y.with(|__s| __s.a) })
        && ({ x.with(|__s| __s.b) } == { y.with(|__s| __s.b) });
}
pub fn operator_ne_1(x: Ptr<S>, y: Ptr<S>) -> bool {
    return !({
        let _x: Ptr<S> = (x).clone();
        let _y: Ptr<S> = (y).clone();
        operator_eq_0(_x, _y)
    });
}
pub fn operator_lt_2(x: Ptr<S>, y: Ptr<S>) -> bool {
    return ({ x.with(|__s| __s.a) } < { y.with(|__s| __s.a) })
        || (({ x.with(|__s| __s.a) } == { y.with(|__s| __s.a) })
            && ({ x.with(|__s| __s.b) } < { y.with(|__s| __s.b) }));
}
pub fn compare_3(x: Ptr<S>, y: Ptr<S>) -> i32 {
    if ({
        let _x: Ptr<S> = (x).clone();
        let _y: Ptr<S> = (y).clone();
        operator_lt_2(_x, _y)
    }) {
        return -1_i32;
    }
    if ({
        let _x: Ptr<S> = (y).clone();
        let _y: Ptr<S> = (x).clone();
        operator_lt_2(_x, _y)
    }) {
        return 1;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
