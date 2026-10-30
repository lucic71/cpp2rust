extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn deref_0(mut p: Ptr<()>) -> () {
    return (p.read());
}
pub fn strlen_1(mut s: Ptr<()>) -> () {
    let c: Value<()> = Rc::new(RefCell::new(0_()));
    'loop_: while ((s.postfix_inc().read()) != 0) {
        (*c.borrow_mut()).prefix_inc();
    }
    return (*c.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_()])));
    let s: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        ('a' as ()),
        ('b' as ()),
        ('c' as ()),
        ('\0' as ()),
    ])));
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
