extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static successful_casts_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static failed_casts_1: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Base {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct FirstDerived {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct SecondDerived {}
pub fn castToFirstDerived_2(base: Ptr<Base>) {
    {
        let derived: Ptr<FirstDerived> = (base).clone();
        (*successful_casts_0.with(Value::clone).borrow_mut()).prefix_inc();
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let first: Value<FirstDerived> = Rc::new(RefCell::new(<FirstDerived>::default()));
    ({ castToFirstDerived_2(first.as_pointer()) });
    assert!(
        (successful_casts_0.with(|rc| *rc.borrow()) == 1)
            && (failed_casts_1.with(|rc| *rc.borrow()) == 0)
    );
    let second: Value<SecondDerived> = Rc::new(RefCell::new(<SecondDerived>::default()));
    ({ castToFirstDerived_2(second.as_pointer()) });
    assert!(
        (successful_casts_0.with(|rc| *rc.borrow()) == 1)
            && (failed_casts_1.with(|rc| *rc.borrow()) == 1)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = successful_casts_0.with(|_| ());
    let _ = failed_casts_1.with(|_| ());
}
