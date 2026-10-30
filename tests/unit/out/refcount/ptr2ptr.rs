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
    let out: Value<()> = Rc::new(RefCell::new(0_()));
    let x: Value<()> = Rc::new(RefCell::new(0_()));
    let p1: Value<Ptr<()>> = Rc::new(RefCell::new((x.as_pointer())));
    (*p1.borrow()).write(1_());
    (*out.borrow_mut()) *= (*x.borrow());
    let p2: Value<Ptr<Ptr<()>>> = Rc::new(RefCell::new((p1.as_pointer())));
    ((*p2.borrow()).read()).write(2_());
    (*out.borrow_mut()) *= (*x.borrow());
    let mut p3: Ptr<Ptr<Ptr<()>>> = (p2.as_pointer());
    ((p3.read()).read()).write(3_());
    (*out.borrow_mut()) *= (*x.borrow());
    return (*out.borrow());
}
pub fn __cpp2rust_init_globals() {}
