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
pub struct Pair {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
impl Pair {
    pub fn copy_from(this: Ptr<Pair>, _a0: Ptr<Pair>) {
        field!(this, first).write(_a0.with(|__s| __s.first));
        field!(this, second).write(_a0.with(|__s| __s.second));
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let __decomp_0: Value<Pair> = Rc::new(RefCell::new(<Pair>::uninit()));
    {
        let __d = __decomp_0.as_pointer();
        field!(__d, first).write(1);
        field!(__d, second).write(2);
    };
    assert!(({ ({ (*__decomp_0.borrow()).first } == 1) }));
    assert!(({ ({ (*__decomp_0.borrow()).second } == 2) }));
    let p: Value<Pair> = Rc::new(RefCell::new(<Pair>::uninit()));
    {
        let __d = p.as_pointer();
        field!(__d, first).write(10);
        field!(__d, second).write(20);
    };
    let __decomp_1: Value<Pair> = Rc::new(RefCell::new(<Pair>::uninit()));
    Pair::copy_from(__decomp_1.as_pointer(), { p.as_pointer() });
    (*__decomp_1.borrow_mut()).first = 11;
    (*__decomp_1.borrow_mut()).second += 1;
    assert!(({ ({ (*__decomp_1.borrow()).first } == 11) }));
    assert!(({ ({ (*__decomp_1.borrow()).second } == 21) }));
    assert!(({ ({ (*p.borrow()).first } == 10) }));
    assert!(({ ({ (*p.borrow()).second } == 20) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
