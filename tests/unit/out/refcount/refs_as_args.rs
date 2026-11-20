extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn more_refs_0(x1: i32, x2: i32, r1: Ptr<i32>, r2: Ptr<i32>) {
    let x1: Value<i32> = Rc::new(RefCell::new(x1));
    let x2: Value<i32> = Rc::new(RefCell::new(x2));
    let rx1: Ptr<i32> = x1.as_pointer();
    let rx2: Ptr<i32> = x2.as_pointer();
    let mut pr1: Ptr<i32> = (r1).clone();
    let mut pr2: Ptr<i32> = (r2).clone();
    let rpr1: Ptr<i32> = (pr1).clone();
    let rpr2: Ptr<i32> = (pr2).clone();
    let r: Ptr<i32> = (r1).clone();
    {
        let __rhs = {
            ({
                ({
                    ({
                        ({ ({ ({ (1 + (rx1.read())) } + { (rx2.read()) }) } + { (pr1.read()) }) }
                            + { (pr2.read()) })
                    } + { (rpr1.read()) })
                } + { (rpr2.read()) })
            } + { (r.read()) })
        };
        rx2.with_mut(|__v| *__v = *__v + __rhs)
    };
    r1.write({ (rx2.read()) });
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Val {
    #[offset(0)]
    pub x: i32,
}
impl Val {
    pub fn new_1(this: Ptr<Val>) {}
    pub fn copy_from(this: Ptr<Val>, _a0: Ptr<Val>) {
        field!(this, x).write(_a0.with(|__s| __s.x));
    }
    pub fn move_from(this: Ptr<Val>, _a0: Ptr<Val>) {
        field!(this, x).write({ (*_a0.upgrade().deref()).x });
    }
}
pub fn sum_1(a: Value<Val>, b: Value<Val>) -> i32 {
    return ({ (*a.borrow()).x } + { (*b.borrow()).x });
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let x1: Value<i32> = Rc::new(RefCell::new(1));
    let x2: Value<i32> = Rc::new(RefCell::new(2));
    ({ more_refs_0(3, 4, x1.as_pointer(), x2.as_pointer()) });
    assert!(({ (((*x1.borrow()) + (*x2.borrow())) == 21) }));
    let v: Value<Val> = Rc::new(RefCell::new(<Val>::uninit()));
    {
        let __d = v.as_pointer();
        field!(__d, x).write(5);
    };
    let mut acc: i32 = ({
        let _a = {
            let __v: Value<Val> = Rc::new(RefCell::new(<Val>::uninit()));
            Val::copy_from(__v.as_pointer(), { v.as_pointer() });
            __v
        };
        let _b = {
            let __v: Value<Val> = Rc::new(RefCell::new(<Val>::uninit()));
            Val::copy_from(__v.as_pointer(), { v.as_pointer() });
            __v
        };
        sum_1(_a, _b)
    });
    acc += ({
        let _a = {
            let __v: Value<Val> = Rc::new(RefCell::new(<Val>::uninit()));
            Val::copy_from(__v.as_pointer(), { v.as_pointer() });
            __v
        };
        let _b = {
            let __v: Value<Val> = Rc::new(RefCell::new(<Val>::uninit()));
            Val::copy_from(__v.as_pointer(), { v.as_pointer() });
            __v
        };
        sum_1(_a, _b)
    });
    acc += ({
        let _a = {
            let __v: Value<Val> = Rc::new(RefCell::new(<Val>::uninit()));
            Val::move_from(__v.as_pointer(), { v.as_pointer() });
            __v
        };
        let _b = {
            let __v: Value<Val> = Rc::new(RefCell::new(<Val>::uninit()));
            Val::copy_from(__v.as_pointer(), { v.as_pointer() });
            __v
        };
        sum_1(_a, _b)
    });
    acc += ({
        let _a = {
            let __v: Value<Val> = Rc::new(RefCell::new(<Val>::uninit()));
            Val::copy_from(__v.as_pointer(), { v.as_pointer() });
            __v
        };
        let _b = {
            let __v: Value<Val> = Rc::new(RefCell::new(<Val>::uninit()));
            Val::copy_from(__v.as_pointer(), { v.as_pointer() });
            __v
        };
        sum_1(_a, _b)
    });
    assert!((acc == 40));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
