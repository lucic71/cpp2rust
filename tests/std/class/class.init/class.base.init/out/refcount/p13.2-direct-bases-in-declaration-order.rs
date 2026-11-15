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
pub fn stampArgument_1(stamp: Ptr<i32>, mut value: i32) -> i32 {
    let __rhs = (*sequence_0.with(Value::clone).borrow_mut()).prefix_inc();
    stamp.write(__rhs);
    return value;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(8)]
pub struct FirstBase {
    #[offset(0)]
    pub value: i32,
    #[offset(4)]
    pub stamp: i32,
}
impl FirstBase {
    pub fn new(mut v: i32) -> Self {
        Self {
            value: v,
            stamp: (*sequence_0.with(Value::clone).borrow_mut()).prefix_inc(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(8)]
pub struct SecondBase {
    #[offset(0)]
    pub value: i32,
    #[offset(4)]
    pub stamp: i32,
}
impl SecondBase {
    pub fn new(mut v: i32) -> Self {
        Self {
            value: v,
            stamp: (*sequence_0.with(Value::clone).borrow_mut()).prefix_inc(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(24)]
pub struct InitializesBasesOutOfOrder {
    #[offset(16)]
    pub firstArgumentStamp: i32,
    #[offset(20)]
    pub secondArgumentStamp: i32,
}
impl InitializesBasesOutOfOrder {
    pub fn new(mut a: i32) -> Self {
        Self {
            firstArgumentStamp: 0_i32,
            secondArgumentStamp: 0_i32,
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<InitializesBasesOutOfOrder> =
        Rc::new(RefCell::new(InitializesBasesOutOfOrder::new({ 10 })));
    assert!(({ ({ ((*d.borrow()) as FirstBase).value } == 12) }));
    assert!(({ ({ ((*d.borrow()) as SecondBase).value } == 11) }));
    assert!(({ ({ (*d.borrow()).firstArgumentStamp } == 1) }));
    assert!(({ ({ ((*d.borrow()) as FirstBase).stamp } == 2) }));
    assert!(({ ({ (*d.borrow()).secondArgumentStamp } == 3) }));
    assert!(({ ({ ((*d.borrow()) as SecondBase).stamp } == 4) }));
    assert!((sequence_0.with(|rc| *rc.borrow()) == 4));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = sequence_0.with(|_| ());
}
