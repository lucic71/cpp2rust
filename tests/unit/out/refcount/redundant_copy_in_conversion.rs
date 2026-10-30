extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sink_0(it: ()) -> () {
    let it: Value<()> = Rc::new(RefCell::new(it));
    let cit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    return if unimplemented!() {
        { ((*it.borrow()).read()).second }
    } else {
        0_()
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!().write(1_());
    let end: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let it0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let const_it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut r: () = if unimplemented!() { 0_() } else { 1_() };
    r += ({ sink_0((*it0.borrow()).clone()) });
    r += if unimplemented!() { 0_() } else { 1_() };
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
