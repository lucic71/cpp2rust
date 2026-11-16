extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Base {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct FirstDerived {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct SecondDerived {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let onlyBase: Value<Base> = Rc::new(RefCell::new(<Base>::default()));
    let mut base: Ptr<Base> = (onlyBase.as_pointer());
    assert!((base).is_null());
    let second: Value<SecondDerived> = Rc::new(RefCell::new(<SecondDerived>::default()));
    base = (second.as_pointer());
    assert!((base).is_null());
    assert!(({ (base).clone() } == { (second.as_pointer()) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
