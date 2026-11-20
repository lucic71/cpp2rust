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
pub struct HasTwoInts {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
impl HasTwoInts {
    pub fn new_1(this: Ptr<HasTwoInts>) {}
    pub fn copy_from(this: Ptr<HasTwoInts>, _a0: Ptr<HasTwoInts>) {
        field!(this, first).write(_a0.with(|__s| __s.first));
        field!(this, second).write(_a0.with(|__s| __s.second));
    }
    pub fn move_from(this: Ptr<HasTwoInts>, _a0: Ptr<HasTwoInts>) {
        field!(this, first).write({ (*_a0.upgrade().deref()).first });
        field!(this, second).write({ (*_a0.upgrade().deref()).second });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let mut pm: isize = -1_isize;
    let obj: Value<HasTwoInts> = Rc::new(RefCell::new(<HasTwoInts>::uninit()));
    {
        let __d = obj.as_pointer();
        field!(__d, first).write(3);
        field!(__d, second).write(4);
    };
    return ((obj.as_pointer()).member::<i32>(pm).read());
}
pub fn __cpp2rust_init_globals() {}
