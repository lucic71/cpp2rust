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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(48)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(24)]
    pub values: (),
    #[offset(24)]
    #[byte_size(24)]
    pub points: (),
}
impl Holder {
    pub fn move_from(_a0: Ptr<Holder>) -> Self {
        Self {
            values: unimplemented!(),
            points: unimplemented!(),
        }
    }
}
pub fn push_and_index_0(v: Ptr<()>) -> () {
    unimplemented!();
    return (unimplemented!() - 1_());
}
pub fn sum_ref_1(v: Ptr<()>) -> () {
    let s: Value<()> = Rc::new(RefCell::new(0_()));
    let mut i: () = 0_();
    'loop_: while ({ i } < { unimplemented!() }) {
        (*s.borrow_mut()) += (unimplemented!().read());
        i.prefix_inc();
    }
    return (*s.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!().write(10_());
    unimplemented!();
    {
        unimplemented!().with_mut(|__v| *__v = *__v + 5_())
    };
    unimplemented!().with_mut(|__v| __v.postfix_inc());
    unimplemented!();
    let __rhs = (unimplemented!().read());
    unimplemented!().write(__rhs);
    unimplemented!();
    unimplemented!().write(0_());
    unimplemented!();
    let mut i: () = 0_();
    unimplemented!().write(3_());
    unimplemented!();
    unimplemented!();
    unimplemented!().write(5_());
    unimplemented!();
    let mut p: Ptr<()> = (unimplemented!());
    p.write(9_());
    unimplemented!();
    unimplemented!();
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::default()));
    unimplemented!();
    unimplemented!().write(6_());
    unimplemented!();
    field!(unimplemented!(), y).write(5_());
    unimplemented!();
    unimplemented!();
    let mut hp: Ptr<Holder> = (h.as_pointer());
    let __rhs = ((unimplemented!().read()) + 1_());
    unimplemented!().write(__rhs);
    let __rhs = (unimplemented!().read());
    field!(unimplemented!(), x).write(__rhs);
    unimplemented!();
    let mut q: Point = (*unimplemented!().upgrade().deref()).clone();
    q.x = 0_();
    unimplemented!();
    let grid: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!().write(8_());
    unimplemented!();
    let __rhs = (unimplemented!().read());
    unimplemented!().write(__rhs);
    let __rhs = (unimplemented!().read());
    {
        unimplemented!().with_mut(|__v| *__v = *__v + __rhs)
    };
    unimplemented!();
    let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let __rhs = ((unimplemented!().read()) + (unimplemented!().read()));
    unimplemented!().write(__rhs);
    unimplemented!();
    return 0_();
}
pub trait HolderImpl {
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder>;
}
impl HolderImpl for Ptr<Holder> {
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder> {
        unimplemented!();
        unimplemented!();
        return (*self).clone();
    }
}
pub trait PointImpl {
    fn sum(&self) -> ();
}
impl PointImpl for Ptr<Point> {
    fn sum(&self) -> () {
        return ((*self).with(|__s| __s.x) + (*self).with(|__s| __s.y));
    }
}
pub fn __cpp2rust_init_globals() {}
