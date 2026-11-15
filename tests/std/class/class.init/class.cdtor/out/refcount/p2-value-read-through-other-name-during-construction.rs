extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(8)]
pub struct ReadsItselfThroughItsName {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
impl ReadsItselfThroughItsName {
    pub fn new(mut a: i32) -> Self {
        Self {
            first: a,
            second: { (*object_0.with(Value::clone).borrow()).first },
        }
    }
}
impl ReadsItselfThroughItsName {}
thread_local!(
    pub static object_0: Value<ReadsItselfThroughItsName> =
        Rc::new(RefCell::new(ReadsItselfThroughItsName::new({ 1 })));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    return { (*object_0.with(Value::clone).borrow()).second };
}
pub fn __cpp2rust_init_globals() {
    let _ = object_0.with(|_| ());
}
