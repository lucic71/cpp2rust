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
pub struct POD {
    #[offset(0)]
    pub x1: i32,
    #[offset(4)]
    pub x2: i32,
    #[offset(8)]
    pub x3: i32,
}
impl POD {
    pub fn copy_from(this: Ptr<POD>, _a0: Ptr<POD>) {
        field!(this, x1).write(_a0.with(|__s| __s.x1));
        field!(this, x2).write(_a0.with(|__s| __s.x2));
        field!(this, x3).write(_a0.with(|__s| __s.x3));
    }
}
pub fn PODIncrement_0(pod: Ptr<POD>) {
    {
        field!(pod, x1).with_mut(|__v| *__v = *__v + 1)
    };
    {
        field!(pod, x2).with_mut(|__v| *__v = *__v + 2)
    };
    {
        field!(pod, x3).with_mut(|__v| *__v = *__v + 3)
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let p1: Value<POD> = Rc::new(RefCell::new(<POD>::uninit()));
    {
        let __d = p1.as_pointer();
        field!(__d, x1).write(10);
        field!(__d, x2).write(11);
        field!(__d, x3).write(12);
    };
    let p2: Value<POD> = Rc::new(RefCell::new(<POD>::uninit()));
    {
        let __d = p2.as_pointer();
        field!(__d, x1).write({ (*p1.borrow()).x1 });
        field!(__d, x2).write({ (*p1.borrow()).x2 });
        field!(__d, x3).write({ (*p1.borrow()).x3 });
    };
    ({ PODIncrement_0(p2.as_pointer()) });
    assert!(
        ({ ((({ (*p2.borrow()).x1 } + { (*p2.borrow()).x2 }) + { (*p2.borrow()).x3 }) == 39) })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
