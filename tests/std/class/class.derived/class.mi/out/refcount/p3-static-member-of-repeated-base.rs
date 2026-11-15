extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static ctorCount_0: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    pub static touchCount_1: Value<i32> = Rc::new(RefCell::new(0));
);
pub type RepeatedBase_Kind = u32;
pub const RepeatedBase_Kind_Plain: RepeatedBase_Kind = 7;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct RepeatedBase {
    #[offset(0)]
    pub value: i32,
}
impl RepeatedBase {
    pub fn new() -> Self {
        let __this: RepeatedBase = Self { value: 0 };
        (*ctorCount_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn touch() {
        (*touchCount_1.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
impl Default for RepeatedBase {
    fn default() -> Self {
        { RepeatedBase::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct FirstPath {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct SecondPath {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct HasTwoRepeatedBases {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct DirectAndIndirect {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<HasTwoRepeatedBases> = Rc::new(RefCell::new(<HasTwoRepeatedBases>::default()));
    assert!((ctorCount_0.with(|rc| *rc.borrow()) == 2));
    ({ RepeatedBase::touch() });
    assert!((touchCount_1.with(|rc| *rc.borrow()) == 1));
    ({ HasTwoRepeatedBasesImpl::touchFromMember(&c.as_pointer()) });
    assert!((touchCount_1.with(|rc| *rc.borrow()) == 2));
    assert!((touchCount_1.with(|rc| *rc.borrow()) == 2));
    assert!(((RepeatedBase_Kind_Plain as i32) == 7));
    let mut d: DirectAndIndirect = <DirectAndIndirect>::default();
    assert!((ctorCount_0.with(|rc| *rc.borrow()) == 4));
    ({ RepeatedBase::touch() });
    assert!((touchCount_1.with(|rc| *rc.borrow()) == 3));
    return 0;
}
pub trait HasTwoRepeatedBasesImpl {
    fn touchFromMember(&self);
}
impl HasTwoRepeatedBasesImpl for Ptr<HasTwoRepeatedBases> {
    fn touchFromMember(&self) {
        ({ RepeatedBase::touch() });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = ctorCount_0.with(|_| ());
    let _ = touchCount_1.with(|_| ());
}
