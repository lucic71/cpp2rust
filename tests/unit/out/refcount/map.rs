extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(mut x: ()) {
    x = { (x).wrapping_add(1_()) };
}
pub fn bar_1(x: Ptr<()>) {
    x.write({ (x.read()).wrapping_add(1_()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!().write(1_());
    unimplemented!().write(2_());
    unimplemented!().write(3_());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = 4_();
    unimplemented!().write(x);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({ foo_0((unimplemented!().read())) });
    unimplemented!();
    ({ bar_1(unimplemented!()) });
    unimplemented!();
    let __rhs = (unimplemented!().read()).wrapping_add((unimplemented!().read()));
    unimplemented!().write(__rhs);
    unimplemented!();
    let end: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let const_it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut x1: () = if unimplemented!() {
        0_()
    } else {
        { ((*it.borrow()).read()).second }
    };
    unimplemented!();
    let mut x2: () = if unimplemented!() {
        0_()
    } else {
        { ((*const_it.borrow()).read()).second }
    };
    unimplemented!();
    let mut x3: () = if unimplemented!() {
        0_()
    } else {
        { ((*it.borrow()).read()).second }
    };
    unimplemented!();
    let mut x4: () = if unimplemented!() {
        0_()
    } else {
        { ((*const_it.borrow()).read()).second }
    };
    unimplemented!();
    unimplemented!().write(5_());
    let it4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut p: Ptr<()> = (field_ptr!(it4.as_pointer(), second));
    let mut x5: () = (p.read());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p.with_mut(|__v| __v.prefix_inc());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let const_it4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let r: Ptr<()> = m.as_pointer();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!().write(7_());
    unimplemented!().write(8_());
    unimplemented!().write(9_());
    unimplemented!();
    unimplemented!();
    let mut key9: () = 9_();
    unimplemented!();
    unimplemented!();
    let cit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let next: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let other_map: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let key0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut value: () = 2_();
    unimplemented!().write(value);
    value = (unimplemented!().read());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut k: () = 0_();
    unimplemented!();
    k.prefix_inc();
    unimplemented!();
    k.prefix_inc();
    unimplemented!();
    let m2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut indexes: () = unimplemented!();
    let mut i: () = 60_();
    'loop_: while (i > 30_()) {
        unimplemented!();
        i.prefix_dec();
    }
    let mut i: () = 100_();
    'loop_: while (i > 60_()) {
        unimplemented!();
        i.prefix_dec();
    }
    let mut i: () = 30_();
    'loop_: while (i > 0_()) {
        unimplemented!();
        i.prefix_dec();
    }
    let mut i: () = 0_();
    'loop_: while (i < unimplemented!()) {
        let __rhs = ((i).wrapping_rem(2_()) != 0);
        unimplemented!().write(__rhs);
        i.prefix_inc();
    }
    unimplemented!();
    let mut last: () = -1_();
    'loop_: for pair in IterRange::new(unimplemented!(), unimplemented!()) {
        let pair: Ptr<()> = unimplemented!();
        unimplemented!();
        unimplemented!();
        last = pair.with(|__s| __s.first);
    }
    k = 0_();
    let value_0: Ptr<()> = unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
