extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Empty {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct FirstPath {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct SecondPath {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(2)]
pub struct HasTwoEmptyBases {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<HasTwoEmptyBases> = Rc::new(RefCell::new(<HasTwoEmptyBases>::default()));
    let mut first: Ptr<FirstPath> = (d.as_pointer());
    let mut second: Ptr<SecondPath> = (d.as_pointer());
    let mut throughFirst: Ptr<Empty> = (first).clone();
    let mut throughSecond: Ptr<Empty> = (second).clone();
    assert!(({ (throughFirst).clone() } != { (throughSecond).clone() }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
