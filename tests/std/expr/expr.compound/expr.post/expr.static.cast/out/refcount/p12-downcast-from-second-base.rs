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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct HasTwoBases {
    #[offset(8)]
    pub own: i32,
}
impl Default for HasTwoBases {
    fn default() -> Self {
        { HasTwoBases { own: 3 } }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<HasTwoBases> = Rc::new(RefCell::new(<HasTwoBases>::default()));
    let mut second: Ptr<SecondBase> = (d.as_pointer());
    let mut derived: Ptr<HasTwoBases> = second.reinterpret_cast::<HasTwoBases>();
    assert!(({ (derived).clone() } == { (d.as_pointer()) }));
    assert!(((derived as *mut FirstBase).with(|__s| __s.first) == 1));
    assert!(((derived as *mut SecondBase).with(|__s| __s.second) == 2));
    assert!((derived.with(|__s| __s.own) == 3));
    field!((derived as *mut FirstBase), first).write(10);
    assert!(({ ({ ((*d.borrow()) as FirstBase).first } == 10) }));
    assert!((second.with(|__s| __s.second) == 2));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
