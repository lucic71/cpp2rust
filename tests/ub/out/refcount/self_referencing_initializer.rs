extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct pair {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
impl pair {
    pub fn new_1(this: Ptr<pair>) {}
    pub fn copy_from(this: Ptr<pair>, _a0: Ptr<pair>) {
        field!(this, a).write(_a0.with(|__s| __s.a));
        field!(this, b).write(_a0.with(|__s| __s.b));
    }
    pub fn move_from(this: Ptr<pair>, _a0: Ptr<pair>) {
        field!(this, a).write({ (*_a0.upgrade().deref()).a });
        field!(this, b).write({ (*_a0.upgrade().deref()).b });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let mut x: i32 = 0_i32;
    x = (x + 1);
    let p: Value<pair> = Rc::new(RefCell::new(<pair>::uninit()));
    {
        let __d = p.as_pointer();
        field!(__d, a).write(1);
        field!(__d, b).write(({ (*p.borrow()).a } + 1));
    };
    return (x + { (*p.borrow()).b });
}
pub fn __cpp2rust_init_globals() {}
