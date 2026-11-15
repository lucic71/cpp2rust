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
pub struct HasCounter {
    #[offset(0)]
    pub count: i32,
}
impl Default for HasCounter {
    fn default() -> Self {
        { HasCounter { count: 0 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct DerivesFromConstTypedef {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<DerivesFromConstTypedef> =
        Rc::new(RefCell::new(<DerivesFromConstTypedef>::default()));
    ({ HasCounterImpl::increment(&(d.as_pointer() as HasCounter)) });
    assert!(({ ({ ((*d.borrow()) as HasCounter).count } == 1) }));
    ({ DerivesFromConstTypedefImpl::incrementTwice(&d.as_pointer()) });
    assert!(({ ({ ((*d.borrow()) as HasCounter).count } == 3) }));
    let mut p: Ptr<HasCounter> = (d.as_pointer());
    ({ HasCounterImpl::increment(&p) });
    assert!(({ ({ ((*d.borrow()) as HasCounter).count } == 4) }));
    return 0;
}
pub trait DerivesFromConstTypedefImpl {
    fn incrementTwice(&self);
}
impl DerivesFromConstTypedefImpl for Ptr<DerivesFromConstTypedef> {
    fn incrementTwice(&self) {
        ({ HasCounterImpl::increment(self) });
        ({ HasCounterImpl::increment(self) });
    }
}
pub trait HasCounterImpl {
    fn increment(&self);
}
impl HasCounterImpl for Ptr<HasCounter> {
    fn increment(&self) {
        field!((*self), count).with_mut(|__v| __v.prefix_inc());
    }
}
pub fn __cpp2rust_init_globals() {}
