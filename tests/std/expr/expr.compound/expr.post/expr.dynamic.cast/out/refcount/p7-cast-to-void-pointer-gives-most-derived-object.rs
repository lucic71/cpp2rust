extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct FirstBase {
    #[offset(8)]
    pub first: i32,
}
impl Default for FirstBase {
    fn default() -> Self {
        { FirstBase { first: 1 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct SecondBase {
    #[offset(8)]
    pub second: i32,
}
impl Default for SecondBase {
    fn default() -> Self {
        { SecondBase { second: 2 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(32)]
pub struct MostDerived {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<MostDerived> = Rc::new(RefCell::new(<MostDerived>::default()));
    let mut second: Ptr<SecondBase> = (d.as_pointer());
    let mut mostDerived: AnyPtr = (second).clone();
    let mut addressOfD: AnyPtr = (d.as_pointer()).to_any();
    let mut addressOfSecond: AnyPtr = (second).to_any();
    assert!(({ (mostDerived).clone() } == { (addressOfD).clone() }));
    assert!(({ (mostDerived).clone() } != { (addressOfSecond).clone() }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
