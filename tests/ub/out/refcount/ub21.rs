extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn strlen_0(mut s: Ptr<()>) -> () {
    let count: Value<()> = Rc::new(RefCell::new(0_()));
    'loop_: while ((s.postfix_inc().read()) != 0) {
        (*count.borrow_mut()).prefix_inc();
    }
    return (*count.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let s: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        ('s' as ()),
        ('t' as ()),
        ('r' as ()),
    ])));
    return ({ strlen_0((s.as_pointer() as Ptr<()>)) });
}
pub fn __cpp2rust_init_globals() {}
