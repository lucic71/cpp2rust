extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(a0: ()) {
    let a0: Value<()> = Rc::new(RefCell::new(a0));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut vec_: () = unimplemented!();
    let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ foo_0((*it.borrow()).clone()) });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
