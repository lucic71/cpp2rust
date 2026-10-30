extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fallthrough_default_0(mut x: (), mut flag: ()) -> () {
    let r: Value<()> = Rc::new(RefCell::new(0_()));
    switch!(match x {
        __v if __v == 7_() => {
            if (flag != 0) {
                (*r.borrow_mut()) = 100_();
                break;
            };
        }
        _ => {
            (*r.borrow_mut()) = 42_();
            break;
        }
    });
    return (*r.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
