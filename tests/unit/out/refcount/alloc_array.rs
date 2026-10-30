extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn All_0(arr: Ptr<()>, mut N: (), mut element: ()) {
    let mut all: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < N) {
        unimplemented!().write(element);
        i.prefix_inc();
    }
    unimplemented!();
}
pub fn Consume_1(mut arr: (), mut N: ()) -> () {
    let sum: Value<()> = Rc::new(RefCell::new(0_()));
    let mut i: () = -1_();
    'loop_: while (i.prefix_inc() < N) {
        (*sum.borrow_mut()) += (unimplemented!().read());
    }
    return (*sum.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut N: () = 10_();
    let arr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ All_0(arr.as_pointer(), N, 1_()) });
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
