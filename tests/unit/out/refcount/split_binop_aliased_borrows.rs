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
    let mut v: () = unimplemented!();
    let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let r: Ptr<()> = unimplemented!();
    let __rhs = (r.read());
    unimplemented!().write(__rhs);
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
