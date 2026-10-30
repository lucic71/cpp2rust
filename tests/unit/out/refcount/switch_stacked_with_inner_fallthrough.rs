extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn stacked_with_inner_fallthrough_0(mut x: (), mut flag: ()) -> () {
    let r: Value<()> = Rc::new(RefCell::new(0_()));
    switch!(match x {
        __v if __v == 1_() || __v == 2_() || __v == 3_() => {
            if !(flag != 0) {
                (*r.borrow_mut()) = 50_();
                break;
            };
        }
        _ => {
            (*r.borrow_mut()) = 999_();
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
