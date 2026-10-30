extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn f_0(bytes: ()) -> () {
    let bytes: Value<()> = Rc::new(RefCell::new(bytes));
    let mut buf: Ptr<()> = Ptr::alloc(unimplemented!());
    let n: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    buf.delete();
    return (*n.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    let mut v: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let l: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
