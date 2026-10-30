extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut N: () = 10000_();
    let mut sentinel: () = (N / 2_());
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let __rhs = sentinel;
    unimplemented!().write(__rhs);
    let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut p: Ptr<()> = (field_ptr!(it.as_pointer(), second));
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < sentinel) {
        let __rhs = i;
        unimplemented!().write(__rhs);
        i.prefix_inc();
    }
    let mut i: () = (sentinel + 1_());
    'loop_: while (i <= N) {
        let __rhs = i;
        unimplemented!().write(__rhs);
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    field!((*it.borrow_mut()), second).write(57005_());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut prev: () = -1_();
    'loop_: for pair in IterRange::new(unimplemented!(), unimplemented!()) {
        let pair: Ptr<()> = unimplemented!();
        unimplemented!();
        prev = pair.with(|__s| __s.first);
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
