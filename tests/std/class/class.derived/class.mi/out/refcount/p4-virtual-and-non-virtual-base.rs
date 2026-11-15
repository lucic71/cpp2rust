extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static repeated_base_ctors_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct RepeatedBase {
    #[offset(0)]
    pub value: i32,
}
impl RepeatedBase {
    pub fn new() -> Self {
        let __this: RepeatedBase = Self { value: 0 };
        (*repeated_base_ctors_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Default for RepeatedBase {
    fn default() -> Self {
        { RepeatedBase::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct FirstVirtualPath {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct SecondVirtualPath {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct NonVirtualPath {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct HasSharedAndOwnBase {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<HasSharedAndOwnBase> = Rc::new(RefCell::new(<HasSharedAndOwnBase>::default()));
    assert!((repeated_base_ctors_0.with(|rc| *rc.borrow()) == 2));
    let mut firstVirtual: Ptr<FirstVirtualPath> = (d.as_pointer());
    let mut secondVirtual: Ptr<SecondVirtualPath> = (d.as_pointer());
    let mut nonVirtual: Ptr<NonVirtualPath> = (d.as_pointer());
    let mut throughFirstVirtual: Ptr<RepeatedBase> = (firstVirtual).clone();
    let mut throughSecondVirtual: Ptr<RepeatedBase> = (secondVirtual).clone();
    let mut throughNonVirtual: Ptr<RepeatedBase> = (nonVirtual).clone();
    assert!(({ (throughFirstVirtual).clone() } == { (throughSecondVirtual).clone() }));
    assert!(({ (throughFirstVirtual).clone() } != { (throughNonVirtual).clone() }));
    (((*d.borrow_mut()) as FirstVirtualPath) as RepeatedBase).value = 1;
    (((*d.borrow_mut()) as NonVirtualPath) as RepeatedBase).value = 2;
    assert!(({ ({ (((*d.borrow()) as SecondVirtualPath) as RepeatedBase).value } == 1) }));
    assert!((throughFirstVirtual.with(|__s| __s.value) == 1));
    assert!((throughSecondVirtual.with(|__s| __s.value) == 1));
    assert!((throughNonVirtual.with(|__s| __s.value) == 2));
    field!(throughSecondVirtual, value).write(3);
    assert!(({ ({ (((*d.borrow()) as FirstVirtualPath) as RepeatedBase).value } == 3) }));
    assert!(({ ({ (((*d.borrow()) as NonVirtualPath) as RepeatedBase).value } == 2) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = repeated_base_ctors_0.with(|_| ());
}
