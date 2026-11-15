extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct FirstBase {
    #[offset(0)]
    pub first: i32,
}
impl Default for FirstBase {
    fn default() -> Self {
        { FirstBase { first: 1 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct SecondBase {
    #[offset(0)]
    pub second: i32,
}
impl Default for SecondBase {
    fn default() -> Self {
        { SecondBase { second: 2 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct HasTwoBases {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<HasTwoBases> = Rc::new(RefCell::new(<HasTwoBases>::default()));
    let mut first: Ptr<FirstBase> = (d.as_pointer());
    let mut second: Ptr<SecondBase> = (d.as_pointer());
    let mut firstAddress: AnyPtr = (first).to_any();
    let mut secondAddress: AnyPtr = (second).to_any();
    return if ({ firstAddress } < { secondAddress }) {
        1
    } else {
        2
    };
}
pub fn __cpp2rust_init_globals() {}
