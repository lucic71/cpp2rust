extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_0(n: (), __args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let n: Value<()> = Rc::new(RefCell::new(n));
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*ap.borrow_mut()) = __args.as_pointer();
    let guard: Value<Guard_1> = Rc::new(RefCell::new(Guard_1::new({ ap.as_pointer() })));
    let _dtor_guard = ScopedDestructor::new(&guard, |__p| __p.destructor());
    let total: Value<()> = Rc::new(RefCell::new(0_()));
    let mut i: () = 0_();
    'loop_: while (i < (*n.borrow())) {
        (*total.borrow_mut()) += { (*guard.borrow()).ap.clone() }.with_mut(|__v| __v.arg::<()>());
        i.prefix_inc();
    }
    return (*total.borrow());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(16)]
pub struct Guard_1 {
    #[offset(0)]
    #[byte_size(8)]
    pub ap: Ptr<Ptr<VaArg>>,
    #[offset(8)]
    pub active: (),
}
impl Guard_1 {
    pub fn new(val: Ptr<Ptr<VaArg>>) -> Self {
        Self {
            ap: (val).clone(),
            active: true,
        }
    }
}
pub fn sum_ptr_2(n: (), __args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let n: Value<()> = Rc::new(RefCell::new(n));
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*ap.borrow_mut()) = __args.as_pointer();
    let mut cursor: Cursor_3 = Cursor_3 {
        ap: (ap.as_pointer()),
    };
    let total: Value<()> = Rc::new(RefCell::new(0_()));
    let mut i: () = 0_();
    'loop_: while (i < (*n.borrow())) {
        (*total.borrow_mut()) += cursor.ap.with_mut(|__v| __v.arg::<()>());
        i.prefix_inc();
    }
    return (*total.borrow());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Cursor_3 {
    #[offset(0)]
    #[byte_size(8)]
    pub ap: Ptr<Ptr<VaArg>>,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait Guard_1Impl {
    fn destructor(&self);
}
impl Guard_1Impl for Ptr<Guard_1> {
    fn destructor(&self) {
        if (*self).with(|__s| __s.active) {}
    }
}
pub fn __cpp2rust_init_globals() {}
