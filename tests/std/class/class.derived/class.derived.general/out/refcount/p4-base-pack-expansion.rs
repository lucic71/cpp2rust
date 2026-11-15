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
pub struct CountsFirst {
    #[offset(0)]
    pub first: i32,
}
impl Default for CountsFirst {
    fn default() -> Self {
        { CountsFirst { first: 0 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct CountsSecond {
    #[offset(0)]
    pub second: i32,
}
impl Default for CountsSecond {
    fn default() -> Self {
        { CountsSecond { second: 0 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct DerivesFromPack_CountsFirst__CountsSecond_ {
    #[offset(8)]
    pub own: i32,
}
impl Default for DerivesFromPack_CountsFirst__CountsSecond_ {
    fn default() -> Self {
        { DerivesFromPack_CountsFirst__CountsSecond_ { own: 0 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct DerivesFromPack_CountsFirst_ {
    #[offset(4)]
    pub own: i32,
}
impl Default for DerivesFromPack_CountsFirst_ {
    fn default() -> Self {
        { DerivesFromPack_CountsFirst_ { own: 0 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct DerivesFromPack__ {
    #[offset(0)]
    pub own: i32,
}
impl Default for DerivesFromPack__ {
    fn default() -> Self {
        { DerivesFromPack__ { own: 0 } }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let both: Value<DerivesFromPack_CountsFirst__CountsSecond_> = Rc::new(RefCell::new(
        <DerivesFromPack_CountsFirst__CountsSecond_>::default(),
    ));
    ({ CountsFirstImpl::touchFirst(&(both.as_pointer() as CountsFirst)) });
    assert!(
        ({
            ({ ((*both.borrow()) as CountsFirst).first } == 1)
                && ({ ((*both.borrow()) as CountsSecond).second } == 0)
        })
    );
    ({ CountsSecondImpl::touchSecond(&(both.as_pointer() as CountsSecond)) });
    assert!(
        ({
            ({ ((*both.borrow()) as CountsFirst).first } == 1)
                && ({ ((*both.borrow()) as CountsSecond).second } == 1)
        })
    );
    let mut second: Ptr<CountsSecond> = (both.as_pointer());
    ({ CountsSecondImpl::touchSecond(&second) });
    assert!(
        ({
            ({ ((*both.borrow()) as CountsFirst).first } == 1)
                && ({ ((*both.borrow()) as CountsSecond).second } == 2)
        })
    );
    let one: Value<DerivesFromPack_CountsFirst_> =
        Rc::new(RefCell::new(<DerivesFromPack_CountsFirst_>::default()));
    ({ CountsFirstImpl::touchFirst(&(one.as_pointer() as CountsFirst)) });
    assert!(({ ({ ((*one.borrow()) as CountsFirst).first } == 1) }));
    assert!(({ ({ ((*both.borrow()) as CountsFirst).first } == 1) }));
    let mut none: DerivesFromPack__ = <DerivesFromPack__>::default();
    none.own = 1;
    assert!((none.own == 1));
    assert!(({ ({ (*both.borrow()).own } == 0) }));
    return 0;
}
pub trait CountsFirstImpl {
    fn touchFirst(&self);
}
impl CountsFirstImpl for Ptr<CountsFirst> {
    fn touchFirst(&self) {
        field!((*self), first).with_mut(|__v| __v.prefix_inc());
    }
}
pub trait CountsSecondImpl {
    fn touchSecond(&self);
}
impl CountsSecondImpl for Ptr<CountsSecond> {
    fn touchSecond(&self) {
        field!((*self), second).with_mut(|__v| __v.prefix_inc());
    }
}
pub fn __cpp2rust_init_globals() {}
