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
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!().write(1_());
    unimplemented!().write(5_());
    unimplemented!().write(6_());
    'loop_: for v2 in IterRange::new(unimplemented!(), unimplemented!()) {
        let v2: Ptr<()> = unimplemented!();
        'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
            let i: Ptr<()> = unimplemented!();
            i.with_mut(|__v| __v.prefix_inc());
        }
    }
    'loop_: for v2 in IterRange::new(unimplemented!(), unimplemented!()) {
        let v2: Ptr<()> = unimplemented!();
        'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut i: () = (unimplemented!().read());
            println!("{}", (i + 3_()));
        }
    }
    'loop_: for v2 in IterRange::new(unimplemented!(), unimplemented!()) {
        let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut i: () = (unimplemented!().read());
            println!("{}", i);
        }
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
