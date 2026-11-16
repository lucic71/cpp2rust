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
pub struct VirtualBase {
    #[offset(8)]
    pub shared: i32,
}
impl Default for VirtualBase {
    fn default() -> Self {
        { VirtualBase { shared: 1 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct FirstPath {
    #[offset(8)]
    pub first: i32,
}
impl Default for FirstPath {
    fn default() -> Self {
        { FirstPath { first: 2 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct SecondPath {
    #[offset(8)]
    pub second: i32,
}
impl Default for SecondPath {
    fn default() -> Self {
        { SecondPath { second: 3 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(48)]
pub struct JoinsPaths {
    #[offset(28)]
    pub own: i32,
}
impl Default for JoinsPaths {
    fn default() -> Self {
        { JoinsPaths { own: 4 } }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<JoinsPaths> = Rc::new(RefCell::new(<JoinsPaths>::default()));
    let mut base: Ptr<VirtualBase> = (d.as_pointer());
    let mut toJoin: Ptr<JoinsPaths> = (base).clone();
    assert!(({ (toJoin).clone() } == { (d.as_pointer()) }));
    assert!((toJoin.with(|__s| __s.own) == 4));
    let mut toSecond: Ptr<SecondPath> = (base).clone();
    let mut expected: Ptr<SecondPath> = (d.as_pointer());
    assert!(({ (toSecond).clone() } == { (expected).clone() }));
    assert!((toSecond.with(|__s| __s.second) == 3));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
