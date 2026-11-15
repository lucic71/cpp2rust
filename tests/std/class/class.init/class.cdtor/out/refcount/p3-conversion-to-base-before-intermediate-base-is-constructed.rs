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
pub struct HasValue {
    #[offset(0)]
    pub value: i32,
}
impl Default for HasValue {
    fn default() -> Self {
        { HasValue { value: 3 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Ptr< HasValue  >  , new, value)]
#[byte_size(16)]
pub struct StoresBasePointer {
    #[offset(8)]
    #[byte_size(8)]
    pub stored: Ptr<HasValue>,
}
impl StoresBasePointer {
    pub fn new(mut base: Ptr<HasValue>) -> Self {
        Self {
            stored: (base).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct ConvertsThisTooEarly {}
impl ConvertsThisTooEarly {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for ConvertsThisTooEarly {
    fn default() -> Self {
        { ConvertsThisTooEarly::new() }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<ConvertsThisTooEarly> = Rc::new(RefCell::new(ConvertsThisTooEarly::new()));
    return { ((*d.borrow()) as StoresBasePointer).stored.clone() }.with(|__s| __s.value);
}
pub fn __cpp2rust_init_globals() {}
