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
#[byte_size(4)]
pub struct FirstPath {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct SecondPath {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct HasTwoRepeatedBases {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<HasTwoRepeatedBases> = Rc::new(RefCell::new(<HasTwoRepeatedBases>::default()));
    assert!((repeated_base_ctors_0.with(|rc| *rc.borrow()) == 2));
    (((*d.borrow_mut()) as FirstPath) as RepeatedBase).value = 1;
    (((*d.borrow_mut()) as SecondPath) as RepeatedBase).value = 2;
    assert!(({ ({ (((*d.borrow()) as FirstPath) as RepeatedBase).value } == 1) }));
    assert!(({ ({ (((*d.borrow()) as SecondPath) as RepeatedBase).value } == 2) }));
    let mut firstPath: Ptr<FirstPath> = (d.as_pointer());
    let mut secondPath: Ptr<SecondPath> = (d.as_pointer());
    let mut baseOfFirst: Ptr<RepeatedBase> = (firstPath).clone();
    let mut baseOfSecond: Ptr<RepeatedBase> = (secondPath).clone();
    assert!(({ (baseOfFirst).clone() } != { (baseOfSecond).clone() }));
    assert!((baseOfFirst.with(|__s| __s.value) == 1));
    assert!((baseOfSecond.with(|__s| __s.value) == 2));
    ({ HasTwoRepeatedBasesImpl::copyValueFromSecondToFirst(&d.as_pointer()) });
    assert!((baseOfFirst.with(|__s| __s.value) == 2));
    assert!((baseOfSecond.with(|__s| __s.value) == 2));
    field!(baseOfSecond, value).write(3);
    assert!(({ ({ (((*d.borrow()) as FirstPath) as RepeatedBase).value } == 2) }));
    assert!(({ ({ (((*d.borrow()) as SecondPath) as RepeatedBase).value } == 3) }));
    return 0;
}
pub trait HasTwoRepeatedBasesImpl {
    fn copyValueFromSecondToFirst(&self);
}
impl HasTwoRepeatedBasesImpl for Ptr<HasTwoRepeatedBases> {
    fn copyValueFromSecondToFirst(&self) {
        field!((((*self) as *mut FirstPath) as *mut RepeatedBase), value)
            .write({ (((*self) as *mut SecondPath) as *mut RepeatedBase).with(|__s| __s.value) });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = repeated_base_ctors_0.with(|_| ());
}
