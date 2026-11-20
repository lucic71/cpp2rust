extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static total_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(1)]
pub struct S {}
impl S {
    pub fn new(this: Ptr<S>, mut x: i32) {
        total_0.with(|rc| *rc.borrow_mut() += x);
    }
    pub fn copy_from(this: Ptr<S>, _a0: Ptr<S>) {}
    pub fn move_from(this: Ptr<S>, _a0: Ptr<S>) {}
}
thread_local!(
    pub static a_1: Value<S> = {
        let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
        S::new(__v.as_pointer(), { 1 });
        __v
    };
);
thread_local!(
    pub static b_2: Value<S> = {
        let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
        S::new(__v.as_pointer(), { 10 });
        __v
    };
);
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    assert!((total_0.with(|rc| *rc.borrow()) == 11));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = total_0.with(|_| ());
    let _ = a_1.with(|_| ());
    let _ = b_2.with(|_| ());
}
