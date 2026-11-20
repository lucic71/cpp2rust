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
pub struct Pair {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl Pair {
    pub fn new_1(this: Ptr<Pair>) {}
    pub fn copy_from(this: Ptr<Pair>, _a0: Ptr<Pair>) {
        field!(this, x).write(_a0.with(|__s| __s.x));
        field!(this, y).write(_a0.with(|__s| __s.y));
    }
    pub fn move_from(this: Ptr<Pair>, _a0: Ptr<Pair>) {
        field!(this, x).write({ (*_a0.upgrade().deref()).x });
        field!(this, y).write({ (*_a0.upgrade().deref()).y });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let mut x: Ptr<i32> = Ptr::alloc(5);
    let mut out: i32 = (x.read());
    x.delete();
    assert!((out == 5));
    let mut y: Ptr<i32> = Ptr::alloc(Default::default());
    y.write(9);
    assert!(((y.read()) == 9));
    y.delete();
    let mut p: Ptr<Pair> = {
        let __p = Ptr::alloc(<Pair>::uninit());
        Pair::new_1(__p.clone());
        __p
    };
    field!(p, x).write(1);
    field!(p, y).write(2);
    assert!((({ p.with(|__s| __s.x) } + { p.with(|__s| __s.y) }) == 3));
    p.delete();
    let mut nullpointer: Ptr<i32> = Ptr::<i32>::null();
    nullpointer.delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
