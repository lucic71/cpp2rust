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
    pub x: u32,
    #[offset(4)]
    pub y: u32,
}
impl Point {
    pub fn new_1(this: Ptr<Point>) {}
    pub fn copy_from(this: Ptr<Point>, _a0: Ptr<Point>) {
        field!(this, x).write(_a0.with(|__s| __s.x));
        field!(this, y).write(_a0.with(|__s| __s.y));
    }
    pub fn move_from(this: Ptr<Point>, _a0: Ptr<Point>) {
        field!(this, x).write({ (*_a0.upgrade().deref()).x });
        field!(this, y).write({ (*_a0.upgrade().deref()).y });
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub first: u32,
    #[offset(4)]
    pub second: u32,
}
impl Pair {
    pub fn new_1(this: Ptr<Pair>) {}
    pub fn copy_from(this: Ptr<Pair>, _a0: Ptr<Pair>) {
        field!(this, first).write(_a0.with(|__s| __s.first));
        field!(this, second).write(_a0.with(|__s| __s.second));
    }
    pub fn move_from(this: Ptr<Pair>, _a0: Ptr<Pair>) {
        field!(this, first).write({ (*_a0.upgrade().deref()).first });
        field!(this, second).write({ (*_a0.upgrade().deref()).second });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let pt: Value<Point> = Rc::new(RefCell::new(<Point>::uninit()));
    {
        let __d = pt.as_pointer();
        field!(__d, x).write(10_u32);
        field!(__d, y).write(20_u32);
    };
    let mut pair: Ptr<Pair> = (pt.as_pointer()).reinterpret_cast::<Pair>();
    assert!((pair.with(|__s| __s.first) == 10_u32));
    assert!((pair.with(|__s| __s.second) == 20_u32));
    field!(pair, first).write(42_u32);
    assert!(({ ({ (*pt.borrow()).x } == 42_u32) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
