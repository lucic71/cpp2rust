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
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<Point> = Rc::new(RefCell::new(<Point>::uninit()));
    Point::new_1(p.as_pointer());
    (*p.borrow_mut()).x = 67305985;
    (*p.borrow_mut()).y = 134678021;
    let mut bytes: Ptr<u8> = (p.as_pointer()).reinterpret_cast::<u8>();
    assert!((((elem!(bytes, 0).read()) as i32) == 1));
    assert!((((elem!(bytes, 3).read()) as i32) == 4));
    assert!((((elem!(bytes, 4).read()) as i32) == 5));
    assert!((((elem!(bytes, 7).read()) as i32) == 8));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
