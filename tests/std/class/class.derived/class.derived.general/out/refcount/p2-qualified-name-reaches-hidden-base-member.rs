extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static base_calls_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static first_calls_1: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static second_calls_2: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct DeclaresTouch {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct HidesTouch {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct HidesTouchAgain {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<HidesTouchAgain> = Rc::new(RefCell::new(<HidesTouchAgain>::default()));
    ({ HidesTouchAgainImpl::touch(&d.as_pointer()) });
    assert!(
        ((base_calls_0.with(|rc| *rc.borrow()) == 0)
            && (first_calls_1.with(|rc| *rc.borrow()) == 0))
            && (second_calls_2.with(|rc| *rc.borrow()) == 1)
    );
    ({ HidesTouchImpl::touch(&(d.as_pointer() as HidesTouch)) });
    assert!(
        ((base_calls_0.with(|rc| *rc.borrow()) == 0)
            && (first_calls_1.with(|rc| *rc.borrow()) == 1))
            && (second_calls_2.with(|rc| *rc.borrow()) == 1)
    );
    ({ DeclaresTouchImpl::touch(&(d.as_pointer() as DeclaresTouch)) });
    assert!(
        ((base_calls_0.with(|rc| *rc.borrow()) == 1)
            && (first_calls_1.with(|rc| *rc.borrow()) == 1))
            && (second_calls_2.with(|rc| *rc.borrow()) == 1)
    );
    ({ HidesTouchAgainImpl::touchAll(&d.as_pointer()) });
    assert!(
        ((base_calls_0.with(|rc| *rc.borrow()) == 2)
            && (first_calls_1.with(|rc| *rc.borrow()) == 2))
            && (second_calls_2.with(|rc| *rc.borrow()) == 2)
    );
    return 0;
}
pub trait DeclaresTouchImpl {
    fn touch(&self);
}
impl DeclaresTouchImpl for Ptr<DeclaresTouch> {
    fn touch(&self) {
        (*base_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub trait HidesTouchImpl {
    fn touch(&self);
}
impl HidesTouchImpl for Ptr<HidesTouch> {
    fn touch(&self) {
        (*first_calls_1.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub trait HidesTouchAgainImpl {
    fn touch(&self);
    fn touchAll(&self);
}
impl HidesTouchAgainImpl for Ptr<HidesTouchAgain> {
    fn touch(&self) {
        (*second_calls_2.with(Value::clone).borrow_mut()).prefix_inc();
    }
    fn touchAll(&self) {
        ({ HidesTouchAgainImpl::touch(self) });
        ({ HidesTouchImpl::touch(self) });
        ({ DeclaresTouchImpl::touch(self) });
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = base_calls_0.with(|_| ());
    let _ = first_calls_1.with(|_| ());
    let _ = second_calls_2.with(|_| ());
}
