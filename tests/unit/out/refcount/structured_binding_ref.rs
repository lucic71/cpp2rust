extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct Triple {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: bool,
    #[offset(8)]
    pub c: i32,
}
impl Triple {
    pub fn new_1(this: Ptr<Triple>) {}
    pub fn copy_from(this: Ptr<Triple>, _a0: Ptr<Triple>) {
        field!(this, a).write(_a0.with(|__s| __s.a));
        field!(this, b).write(_a0.with(|__s| __s.b));
        field!(this, c).write(_a0.with(|__s| __s.c));
    }
    pub fn move_from(this: Ptr<Triple>, _a0: Ptr<Triple>) {
        field!(this, a).write({ (*_a0.upgrade().deref()).a });
        field!(this, b).write({ (*_a0.upgrade().deref()).b });
        field!(this, c).write({ (*_a0.upgrade().deref()).c });
    }
}
pub fn sum_0(t: Ptr<Triple>) -> i32 {
    let __decomp_1: Ptr<Triple> = (t).clone();
    return ((__decomp_1.with(|__s| __s.a) + (if __decomp_1.with(|__s| __s.b) { 1 } else { 0 }))
        + __decomp_1.with(|__s| __s.c));
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let t: Value<Triple> = Rc::new(RefCell::new(<Triple>::uninit()));
    {
        let __d = t.as_pointer();
        field!(__d, a).write(10);
        field!(__d, b).write(false);
        field!(__d, c).write(20);
    };
    let __decomp_2: Ptr<Triple> = t.as_pointer();
    field!(__decomp_2, a).write(11);
    field!(__decomp_2, b).write(true);
    {
        field!(__decomp_2, c).with_mut(|__v| *__v = *__v + 1)
    };
    assert!(({ ({ (*t.borrow()).a } == 11) }));
    assert!(({ { (*t.borrow()).b } }));
    assert!(({ ({ (*t.borrow()).c } == 21) }));
    (*t.borrow_mut()).a = 12;
    assert!((__decomp_2.with(|__s| __s.a) == 12));
    let __decomp_3: Ptr<Triple> = t.as_pointer();
    (*t.borrow_mut()).c = 30;
    assert!((__decomp_3.with(|__s| __s.a) == 12));
    assert!(__decomp_3.with(|__s| __s.b));
    assert!((__decomp_3.with(|__s| __s.c) == 30));
    assert!((({ sum_0(t.as_pointer(),) }) == 43));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
