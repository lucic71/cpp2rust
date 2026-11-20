extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct S_int_ {
    #[offset(0)]
    pub x: i32,
}
impl S_int_ {
    pub fn new_1(this: Ptr<S_int_>) {
        field!(this, x).write(0);
    }
    pub fn copy_from(this: Ptr<S_int_>, _a0: Ptr<S_int_>) {
        field!(this, x).write(_a0.with(|__s| __s.x));
    }
}
impl Default for S_int_ {
    fn default() -> Self {
        { S_int_ { x: 0 } }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<S_int_> = Rc::new(RefCell::new(<S_int_>::uninit()));
    S_int_::new_1(p.as_pointer());
    ({ S_int_Impl::set(&p.as_pointer(), 3) });
    assert!((({ f_0((p.as_pointer()),) }) == 3));
    return 0;
}
pub fn f_0(mut p: Ptr<S_int_>) -> i32 {
    return ({ S_int_Impl::get(&p) });
}
pub trait S_int_Impl {
    fn set(&self, v: i32);
    fn get(&self) -> i32 {
        unimplemented!()
    }
}
impl S_int_Impl for Ptr<S_int_> {
    fn set(&self, mut v: i32) {
        field!((*self), x).write(v);
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.x);
    }
}
pub fn __cpp2rust_init_globals() {}
