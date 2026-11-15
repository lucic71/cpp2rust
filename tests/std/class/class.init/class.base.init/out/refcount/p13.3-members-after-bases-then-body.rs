extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static sequence_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new_1, value)]
#[byte_size(4)]
pub struct StampsConstruction {
    #[offset(0)]
    pub stamp: i32,
}
impl StampsConstruction {
    pub fn new_2() -> Self {
        Self {
            stamp: (*sequence_0.with(Value::clone).borrow_mut()).prefix_inc(),
        }
    }
    pub fn new_1(mut _a0: i32) -> Self {
        Self {
            stamp: (*sequence_0.with(Value::clone).borrow_mut()).prefix_inc(),
        }
    }
}
impl Default for StampsConstruction {
    fn default() -> Self {
        { StampsConstruction::new_2() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct FirstBase {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct SecondBase {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct InitializesMembersOutOfOrder {
    #[offset(8)]
    #[byte_size(4)]
    pub firstMember: StampsConstruction,
    #[offset(12)]
    #[byte_size(4)]
    pub secondMember: StampsConstruction,
    #[offset(16)]
    #[byte_size(4)]
    pub thirdMember: StampsConstruction,
    #[offset(20)]
    pub bodyStamp: i32,
}
impl InitializesMembersOutOfOrder {
    pub fn new() -> Self {
        let __this: Value<InitializesMembersOutOfOrder> = Rc::new(RefCell::new(Self {
            firstMember: StampsConstruction::new_1({ 0 }),
            secondMember: StampsConstruction::new_1({ 0 }),
            thirdMember: StampsConstruction::new_1({ 0 }),
            bodyStamp: 0_i32,
        }));
        let this: Ptr<InitializesMembersOutOfOrder> = __this.as_pointer();
        field!(this, bodyStamp).write((*sequence_0.with(Value::clone).borrow_mut()).prefix_inc());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for InitializesMembersOutOfOrder {
    fn default() -> Self {
        { InitializesMembersOutOfOrder::new() }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<InitializesMembersOutOfOrder> =
        Rc::new(RefCell::new(InitializesMembersOutOfOrder::new()));
    assert!(({ ({ (((*d.borrow()) as FirstBase) as StampsConstruction).stamp } == 1) }));
    assert!(({ ({ (((*d.borrow()) as SecondBase) as StampsConstruction).stamp } == 2) }));
    assert!(({ ({ (*d.borrow()).firstMember.stamp } == 3) }));
    assert!(({ ({ (*d.borrow()).secondMember.stamp } == 4) }));
    assert!(({ ({ (*d.borrow()).thirdMember.stamp } == 5) }));
    assert!(({ ({ (*d.borrow()).bodyStamp } == 6) }));
    assert!((sequence_0.with(|rc| *rc.borrow()) == 6));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = sequence_0.with(|_| ());
}
