extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Point {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub y: (),
}
pub fn set_0(ref_: (), mut val: ()) {
    let ref_: Value<()> = Rc::new(RefCell::new(ref_));
    unimplemented!().write(val);
}
pub fn read_1(ref_: ()) -> () {
    let ref_: Value<()> = Rc::new(RefCell::new(ref_));
    let r: Ptr<()> = unimplemented!();
    return (r.read());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut i1: () = 10_();
    let ref_1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!().write(20_());
    let i2: Ptr<()> = unimplemented!();
    {
        i2.with_mut(|__v| *__v = *__v + 5_())
    };
    unimplemented!();
    let mut i3: () = 1_();
    let mut i4: () = 2_();
    let ref_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ref_4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let __rhs = (unimplemented!().read());
    unimplemented!().write(__rhs);
    unimplemented!();
    ({ set_0(unimplemented!(), 99_()) });
    unimplemented!();
    let mut point: Point = Point { x: 3_(), y: 4_() };
    let point_ref: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    field!(unimplemented!(), x).write(30_());
    field!(unimplemented!(), y).write(40_());
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
