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
pub struct VirtualBase {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Sibling {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Ptr< Sibling  >  , new, value)]
#[byte_size(16)]
pub struct CastsSiblingInConstructor {
    #[offset(8)]
    pub castSucceeded: bool,
}
impl CastsSiblingInConstructor {
    pub fn new(mut sibling: Ptr<Sibling>) -> Self {
        Self {
            castSucceeded: !((sibling).is_null()),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct MostDerived {}
impl MostDerived {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for MostDerived {
    fn default() -> Self {
        { MostDerived::new() }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<MostDerived> = Rc::new(RefCell::new(MostDerived::new()));
    return if { ((*d.borrow()) as CastsSiblingInConstructor).castSucceeded } {
        1
    } else {
        2
    };
}
pub fn __cpp2rust_init_globals() {}
