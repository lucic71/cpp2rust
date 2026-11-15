extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static first_calls_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static second_calls_1: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub trait HasPureVirtual {
    fn callPure(&self) {
        ({ (*(*self).upgrade().deref_mut()).pure() });
    }
    fn pure(&mut self);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct FirstOverrider {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct SecondOverrider {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let first: Value<FirstOverrider> = Rc::new(RefCell::new(<FirstOverrider>::default()));
    let second: Value<SecondOverrider> = Rc::new(RefCell::new(<SecondOverrider>::default()));
    let mut p: PtrDyn<dyn HasPureVirtual> =
        (first.as_pointer()).to_dyn::<dyn HasPureVirtual>(|w| w);
    ({ HasPureVirtualImpl::callPure(&p) });
    assert!(
        (first_calls_0.with(|rc| *rc.borrow()) == 1)
            && (second_calls_1.with(|rc| *rc.borrow()) == 0)
    );
    p = (second.as_pointer()).to_dyn::<dyn HasPureVirtual>(|w| w);
    ({ HasPureVirtualImpl::callPure(&p) });
    assert!(
        (first_calls_0.with(|rc| *rc.borrow()) == 1)
            && (second_calls_1.with(|rc| *rc.borrow()) == 1)
    );
    ({ (*p.upgrade().deref_mut()).pure() });
    assert!(
        (first_calls_0.with(|rc| *rc.borrow()) == 1)
            && (second_calls_1.with(|rc| *rc.borrow()) == 2)
    );
    return 0;
}
impl HasPureVirtual for FirstOverrider {
    fn pure(&mut self) {
        (*first_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
impl HasPureVirtual for SecondOverrider {
    fn pure(&mut self) {
        (*second_calls_1.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = first_calls_0.with(|_| ());
    let _ = second_calls_1.with(|_| ());
}
