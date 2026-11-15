extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static base_dtors_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
thread_local!(
    pub static derived_dtors_1: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(1)]
pub struct HasNonVirtualDestructor {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(1)]
pub struct DerivesWithOwnDestructor {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut p: Ptr<HasNonVirtualDestructor> = Ptr::alloc(<DerivesWithOwnDestructor>::default());
    {
        let __p = p;
        __p.destructor();
        __p.delete();
    };
    return (base_dtors_0.with(|rc| *rc.borrow()) + (2 * derived_dtors_1.with(|rc| *rc.borrow())));
}
pub trait DerivesWithOwnDestructorImpl {
    fn destructor(&self);
}
impl DerivesWithOwnDestructorImpl for Ptr<DerivesWithOwnDestructor> {
    fn destructor(&self) {
        (*derived_dtors_1.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub trait HasNonVirtualDestructorImpl {
    fn destructor(&self);
}
impl HasNonVirtualDestructorImpl for Ptr<HasNonVirtualDestructor> {
    fn destructor(&self) {
        (*base_dtors_0.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = base_dtors_0.with(|_| ());
    let _ = derived_dtors_1.with(|_| ());
}
