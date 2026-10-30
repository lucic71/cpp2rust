extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn dowhile_0(x: ()) -> () {
    let x: Value<()> = Rc::new(RefCell::new(x));
    let mut __do_while = true;
    'loop_: while __do_while || ((*x.borrow()) <= 200_()) {
        __do_while = false;
        (*x.borrow_mut()) += 1_();
        let mut __do_while = true;
        'loop_: while __do_while || ((*x.borrow()) <= 100_()) {
            __do_while = false;
            (*x.borrow_mut()) += 1_();
            (*x.borrow_mut()) += 1_();
        }
        (*x.borrow_mut()) += 1_();
    }
    return (*x.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
