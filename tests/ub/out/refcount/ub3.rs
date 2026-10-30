extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn dangling_0() -> Ptr<()> {
    let x: Value<()> = Rc::new(RefCell::new(1_()));
    let p: Value<Ptr<()>> = Rc::new(RefCell::new((x.as_pointer())));
    return (*p.borrow()).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut x: Ptr<()> = ({ dangling_0() });
    return (x.read());
}
pub fn __cpp2rust_init_globals() {}
