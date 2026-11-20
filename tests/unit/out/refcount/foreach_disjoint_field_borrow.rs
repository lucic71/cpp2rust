extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(32)]
pub struct S {
    #[offset(0)]
    #[byte_size(24)]
    pub v: Vec<i32>,
    #[offset(24)]
    pub a: i32,
}
impl S {
    pub fn copy_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, v).write((*_a0.upgrade().deref()).v.clone());
        field!(this, a).write(_a0.with(|__s| __s.a));
    }
    pub fn move_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, v).write(field!(_a0, v).with_mut(std::mem::take));
        field!(this, a).write({ (*_a0.upgrade().deref()).a });
    }
    pub fn new_1(this: Ptr<S>) {
        field!(this, v).write(Vec::new());
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    S::new_1(s.as_pointer());
    {
        let __a1 = 1;
        (*s.borrow_mut()).v.push(__a1)
    };
    'loop_: for e in IterRange::new(
        field_ref!(s.as_pointer(), v).decay().begin(),
        field_ref!(s.as_pointer(), v).decay().end(),
    ) {
        let mut e: i32 = (e.iter_deref().read());
        (*s.borrow_mut()).a.postfix_inc();
    }
    return 0;
}
pub trait SImpl {
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S>;
}
impl SImpl for Ptr<S> {
    fn move_assign(&self, _a0: Ptr<S>) -> Ptr<S> {
        field!((*self), v).with_mut(|__b0| {
            (*__b0) = field!(_a0, v).with_mut(std::mem::take);
        });
        field!((*self), a).write({ { (*_a0.upgrade().deref()).a } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
