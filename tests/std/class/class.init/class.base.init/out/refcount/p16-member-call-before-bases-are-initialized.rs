extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct HasIntConstructor {
    #[offset(0)]
    pub value: i32,
}
impl HasIntConstructor {
    pub fn new(mut v: i32) -> Self {
        Self { value: v }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct CallsMemberBeforeBaseIsInitialized {
    #[offset(4)]
    pub own: i32,
}
impl CallsMemberBeforeBaseIsInitialized {
    pub fn new() -> Self {
        Self { own: 5 }
    }
}
impl Default for CallsMemberBeforeBaseIsInitialized {
    fn default() -> Self {
        { CallsMemberBeforeBaseIsInitialized::new() }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<CallsMemberBeforeBaseIsInitialized> =
        Rc::new(RefCell::new(CallsMemberBeforeBaseIsInitialized::new()));
    return { ((*d.borrow()) as HasIntConstructor).value };
}
pub trait CallsMemberBeforeBaseIsInitializedImpl {
    fn readOwn(&self) -> i32;
}
impl CallsMemberBeforeBaseIsInitializedImpl for Ptr<CallsMemberBeforeBaseIsInitialized> {
    fn readOwn(&self) -> i32 {
        return (*self).with(|__s| __s.own);
    }
}
pub fn __cpp2rust_init_globals() {}
