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
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    let mut k: () = 100_();
    'loop_: while (i < 100_()) {
        unimplemented!().write((k / 2_f64));
        {
            i.prefix_inc();
            k.prefix_dec()
        };
    }
    let mut sum: () = 0_();
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let i: Ptr<()> = unimplemented!();
        sum += i.with(|__s| __s.second);
    }
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        sum += { (*i.borrow()).first };
    }
    unimplemented!();
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*i.borrow_mut()).second = -1_();
        unimplemented!();
    }
    'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
        let i: Ptr<()> = unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
