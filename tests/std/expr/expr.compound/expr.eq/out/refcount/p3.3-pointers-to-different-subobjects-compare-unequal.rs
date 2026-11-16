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
    let a: Value<HasTwoBases> = Rc::new(RefCell::new(<HasTwoBases>::default()));
    let b: Value<HasTwoBases> = Rc::new(RefCell::new(<HasTwoBases>::default()));
    let mut secondOfA: Ptr<SecondBase> = (a.as_pointer());
    let mut secondOfB: Ptr<SecondBase> = (b.as_pointer());
    assert!(({ (secondOfA).clone() } != { (secondOfB).clone() }));
    assert!(!({ (secondOfA).clone() } == { (secondOfB).clone() }));
    let mut firstOfA: Ptr<FirstBase> = (a.as_pointer());
    let mut firstAddress: AnyPtr = (firstOfA).to_any();
    let mut secondAddress: AnyPtr = (secondOfA).to_any();
    assert!(({ (firstAddress).clone() } != { (secondAddress).clone() }));
    let mut nullSecond: Ptr<SecondBase> = Ptr::<SecondBase>::null();
    assert!(({ (secondOfA).clone() } != { (nullSecond).clone() }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
