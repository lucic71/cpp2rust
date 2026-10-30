extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn median_0(mut w: (), mut n: (), mut nw: ()) -> () {
    let mx: Value<()> = Rc::new(RefCell::new(if (w > n) { w } else { n }));
    let mn: Value<()> = Rc::new(RefCell::new(((w + n) - (*mx.borrow()))));
    if (nw > (*mx.borrow())) {
        return (*mn.borrow());
    } else if (nw < (*mn.borrow())) {
        return (*mx.borrow());
    }
    return ((n + w) - nw);
}
pub fn assign_1(mut c: (), mut v: ()) -> () {
    let a: Value<()> = Rc::new(RefCell::new(1_()));
    let b: Value<()> = Rc::new(RefCell::new(2_()));
    if c {
        (a.as_pointer())
    } else {
        (b.as_pointer())
    }
    .write(v);
    if c {
        (a.as_pointer())
    } else {
        (b.as_pointer())
    }
    .with_mut(|__v| __v.prefix_inc());
    return (((*a.borrow()) * 10_()) + (*b.borrow()));
}
pub fn lehmer_2(mut sigma: Ptr<()>, mut len: (), mut code: Ptr<()>) {
    let mut items: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < len) {
        let __rhs = i;
        unimplemented!().write(__rhs);
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i < len) {
        let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let __rhs = unimplemented!();
        elem!(code, i).write(__rhs);
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn sum_3(v: Ptr<()>) -> () {
    let s: Value<()> = Rc::new(RefCell::new(0_()));
    let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    'loop_: while !(unimplemented!()) {
        (*s.borrow_mut()) += (unimplemented!().read());
        unimplemented!();
    }
    return (*s.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let sigma: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([2_(), 0_(), 3_(), 1_()])));
    let code: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    ({
        lehmer_2(
            (sigma.as_pointer() as Ptr<()>),
            4_(),
            (code.as_pointer() as Ptr<()>),
        )
    });
    unimplemented!();
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
