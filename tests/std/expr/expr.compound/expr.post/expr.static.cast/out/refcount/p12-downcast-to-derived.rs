extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static derived_only_calls_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct Base {
    #[offset(8)]
    pub base: i32,
}
impl Default for Base {
    fn default() -> Self {
        { Base { base: 1 } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct Derived {
    #[offset(12)]
    pub derived: i32,
}
impl Default for Derived {
    fn default() -> Self {
        { Derived { derived: 2 } }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<Derived> = Rc::new(RefCell::new(<Derived>::default()));
    let mut base: Ptr<Base> = (d.as_pointer());
    let mut derived: Ptr<Derived> = base.reinterpret_cast::<Derived>();
    assert!(({ (derived).clone() } == { (d.as_pointer()) }));
    assert!((derived.with(|__s| __s.derived) == 2));
    ({ DerivedImpl::derivedOnly(&derived) });
    assert!((derived_only_calls_0.with(|rc| *rc.borrow()) == 1));
    field!(derived, derived).write(5);
    assert!(({ ({ (*d.borrow()).derived } == 5) }));
    assert!(({ ({ ((*d.borrow()) as Base).base } == 1) }));
    return 0;
}
pub trait DerivedImpl {
    fn derivedOnly(&self);
}
impl DerivedImpl for Ptr<Derived> {
    fn derivedOnly(&self) {
        (*derived_only_calls_0.with(Value::clone).borrow_mut()).prefix_inc();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = derived_only_calls_0.with(|_| ());
}
