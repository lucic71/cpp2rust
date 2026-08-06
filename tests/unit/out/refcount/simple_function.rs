extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0() -> i32 {
    return 0;
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new((({ foo_0() }) + 1)));
    let y: Value<i32> = Rc::new(RefCell::new(({ foo_0() })));
    return ((*x.borrow()) + (*y.borrow()));
}
