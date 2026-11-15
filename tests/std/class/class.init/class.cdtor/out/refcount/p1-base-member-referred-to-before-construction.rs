extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct HasMember {
    #[offset(0)]
    pub value: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct HasNonTrivialConstructor {}
impl HasNonTrivialConstructor {
    pub fn new() -> Self {
        let __this: Value<HasNonTrivialConstructor> = Rc::new(RefCell::new(Self {}));
        let this: Ptr<HasNonTrivialConstructor> = __this.as_pointer();
        field!((this as *mut HasMember), value).write(7);
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for HasNonTrivialConstructor {
    fn default() -> Self {
        { HasNonTrivialConstructor::new() }
    }
}
thread_local!(
    pub static formedBeforeConstruction_1: Value<Ptr<i32>> = Rc::new(RefCell::new(
        (field_ptr!((object_0.with(|v| v.as_pointer()) as HasMember), value)),
    ));
);
thread_local!(
    pub static object_0: Value<HasNonTrivialConstructor> =
        Rc::new(RefCell::new(HasNonTrivialConstructor::new()));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    return ((*formedBeforeConstruction_1.with(Value::clone).borrow()).read());
}
pub fn __cpp2rust_init_globals() {
    let _ = formedBeforeConstruction_1.with(|_| ());
    let _ = object_0.with(|_| ());
}
