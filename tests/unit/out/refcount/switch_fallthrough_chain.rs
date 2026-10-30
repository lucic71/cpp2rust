extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fallthrough_chain_0(mut x: ()) -> () {
    let r: Value<()> = Rc::new(RefCell::new(0_()));
    switch!(match x {
        __v if __v == 1_() => {
            (*r.borrow_mut()) += 1_();
        }
        __v if __v == 2_() => {
            (*r.borrow_mut()) += 2_();
        }
        __v if __v == 3_() => {
            (*r.borrow_mut()) += 4_();
        }
        __v if __v == 4_() => {
            (*r.borrow_mut()) += 8_();
            break;
        }
        _ => {
            (*r.borrow_mut()) = -1_();
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
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
