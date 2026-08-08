extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static buf_0: Value<Box<[u8]>> = Rc::new(RefCell::new(
        (0..16).map(|_| <u8>::default()).collect::<Box<[u8]>>(),
    ));
);
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let p: Value<Ptr<u8>> = Rc::new(RefCell::new(
        (buf_0.with(Value::clone).as_pointer() as Ptr<u8>),
    ));
    return ((((((*p.borrow()).clone() - Ptr::<u8>::null()) as i64) & 7_i64) == 0_i64) as i32);
}
