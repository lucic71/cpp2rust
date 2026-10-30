extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn no_default_0(mut x: ()) -> () {
    let r: Value<()> = Rc::new(RefCell::new(-1_()));
    'switch: {
        match { x } {
            __v if __v == 7_() => {
                (*r.borrow_mut()) = 1_();
                break 'switch;
            }
            __v if __v == 8_() => {
                (*r.borrow_mut()) = 2_();
                break 'switch;
            }
            _ => {}
        }
    };
    return (*r.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
