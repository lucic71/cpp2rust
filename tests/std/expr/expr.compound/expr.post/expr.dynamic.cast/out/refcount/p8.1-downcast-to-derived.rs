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
pub struct Base {
    #[offset(8)]
    pub base: i32,
}
impl Default for Base {
    fn default() -> Self {
        { Base { base: 1 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct Intermediate {
    #[offset(12)]
    pub intermediate: i32,
}
impl Default for Intermediate {
    fn default() -> Self {
        { Intermediate { intermediate: 2 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct MostDerived {
    #[offset(16)]
    pub mostDerived: i32,
}
impl Default for MostDerived {
    fn default() -> Self {
        { MostDerived { mostDerived: 3 } }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<MostDerived> = Rc::new(RefCell::new(<MostDerived>::default()));
    let mut base: Ptr<Base> = (d.as_pointer());
    let mut toMostDerived: Ptr<MostDerived> = (base).clone();
    assert!(({ (toMostDerived).clone() } == { (d.as_pointer()) }));
    assert!((toMostDerived.with(|__s| __s.mostDerived) == 3));
    let mut toIntermediate: Ptr<Intermediate> = (base).clone();
    assert!(({ (toIntermediate).clone() } == { (d.as_pointer()) }));
    assert!((toIntermediate.with(|__s| __s.intermediate) == 2));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
