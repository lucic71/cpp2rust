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
pub struct Base {
    #[offset(0)]
    pub base: i32,
}
impl Default for Base {
    fn default() -> Self {
        { Base { base: 1 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Derived {
    #[offset(4)]
    pub derived: i32,
}
impl Default for Derived {
    fn default() -> Self {
        { Derived { derived: 2 } }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let onlyBase: Value<Base> = Rc::new(RefCell::new(<Base>::default()));
    let mut base: Ptr<Base> = (onlyBase.as_pointer());
    let mut derived: Ptr<Derived> = base.reinterpret_cast::<Derived>();
    return derived.with(|__s| __s.derived);
}
pub fn __cpp2rust_init_globals() {}
