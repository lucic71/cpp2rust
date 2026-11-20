extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub v: i32,
}
impl S {
    pub fn new_1(this: Ptr<S>) {}
    pub fn copy_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
    pub fn move_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, v).write({ (*_a0.upgrade().deref()).v });
    }
}
pub fn operator_not_0(a: Ptr<S>) -> bool {
    return (a.with(|__s| __s.v) == 0);
}
pub fn operator_and_1(a: Ptr<S>, b: Ptr<S>) -> bool {
    return (a.with(|__s| __s.v) != 0) && (b.with(|__s| __s.v) != 0);
}
pub fn operator_or_2(a: Ptr<S>, b: Ptr<S>) -> bool {
    return (a.with(|__s| __s.v) != 0) || (b.with(|__s| __s.v) != 0);
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let t: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = t.as_pointer();
        field!(__d, v).write(1);
    };
    let f: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = f.as_pointer();
        field!(__d, v).write(0);
    };
    assert!(
        ({
            let _a: Ptr<S> = f.as_pointer();
            operator_not_0(_a)
        })
    );
    assert!(
        !({
            let _a: Ptr<S> = t.as_pointer();
            operator_not_0(_a)
        })
    );
    assert!(
        ({
            let _a: Ptr<S> = t.as_pointer();
            let _b: Ptr<S> = t.as_pointer();
            operator_and_1(_a, _b)
        })
    );
    assert!(
        !({
            let _a: Ptr<S> = t.as_pointer();
            operator_and_1(_a, f.as_pointer())
        })
    );
    assert!(
        ({
            let _a: Ptr<S> = t.as_pointer();
            operator_or_2(_a, f.as_pointer())
        })
    );
    assert!(
        !({
            let _a: Ptr<S> = f.as_pointer();
            let _b: Ptr<S> = f.as_pointer();
            operator_or_2(_a, _b)
        })
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
