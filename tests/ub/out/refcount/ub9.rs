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
    let mut arr: Ptr<()> =
        Ptr::alloc_array((0..10_()).map(|_| unimplemented!()).collect::<Box<[()]>>());
    let out: Value<()> = Rc::new(RefCell::new((elem!(arr, 10_()).read())));
    arr.delete();
    return (*out.borrow());
}
pub fn __cpp2rust_init_globals() {}
