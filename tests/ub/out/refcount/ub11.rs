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
    let mut element: Ptr<()> = Ptr::alloc(10_());
    let mut ptr: Ptr<()> = element.offset((1_()) as isize);
    let out: Value<()> = Rc::new(RefCell::new((ptr.read())));
    element.delete();
    return (*out.borrow());
}
pub fn __cpp2rust_init_globals() {}
