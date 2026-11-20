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
pub struct Point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl Point {
    pub fn copy_from(this: Ptr<Point>, _a0: Ptr<Point>) {
        field!(this, x).write(_a0.with(|__s| __s.x));
        field!(this, y).write(_a0.with(|__s| __s.y));
    }
    pub fn move_from(this: Ptr<Point>, _a0: Ptr<Point>) {
        field!(this, x).write({ (*_a0.upgrade().deref()).x });
        field!(this, y).write({ (*_a0.upgrade().deref()).y });
    }
}
pub fn sum_0(p: Value<Point>) -> i32 {
    return ({ (*p.borrow()).x } + { (*p.borrow()).y });
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let mut p: Option<Value<Point>> = Some(Rc::new(RefCell::new(
        Rc::try_unwrap({
            let __v: Value<Point> = Rc::new(RefCell::new(<Point>::uninit()));
            {
                let __d = __v.as_pointer();
                field!(__d, x).write(3);
                field!(__d, y).write(4);
            };
            __v
        })
        .ok()
        .unwrap()
        .into_inner(),
    )));
    {
        field!(p.as_pointer(), x).with_mut(|__v| *__v = *__v + 10)
    };
    let __rhs = (p.as_pointer().with(|__s| __s.x) + p.as_pointer().with(|__s| __s.y));
    field!(p.as_pointer(), y).write(__rhs);
    let mut s: i32 = ({
        sum_0({
            let __v: Value<Point> = Rc::new(RefCell::new(<Point>::uninit()));
            Point::copy_from(__v.as_pointer(), { p.as_pointer() });
            __v
        })
    });
    assert!((s == 30));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
