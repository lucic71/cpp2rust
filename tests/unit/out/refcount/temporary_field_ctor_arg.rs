extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(24)]
    pub values: Vec<i32>,
}
impl Outer {
    pub fn copy_from(this: Ptr<Outer>, _a0: Ptr<Outer>) {
        field!(this, values).write((*_a0.upgrade().deref()).values.clone());
    }
    pub fn move_from(this: Ptr<Outer>, _a0: Ptr<Outer>) {
        field!(this, values).write(field!(_a0, values).with_mut(std::mem::take));
    }
}
pub fn make_0(__ret: Ptr<Outer>) {
    {
        let __d = __ret.clone();
        field!(__d, values).write(Vec::from([1, 2, 3, 4]));
    };
    return;
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Vec<i32  >  , new, ptr)]
#[byte_size(4)]
pub struct H {
    #[offset(0)]
    pub last: i32,
}
impl H {
    pub fn new(this: Ptr<H>, values: Ptr<Vec<i32>>) {
        field!(this, last).write((index!(values, 3_usize).read()));
    }
    pub fn copy_from(this: Ptr<H>, _a0: Ptr<H>) {
        field!(this, last).write(_a0.with(|__s| __s.last));
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let h: Value<H> = Rc::new(RefCell::new(<H>::uninit()));
    H::new(h.as_pointer(), {
        (match {
            let __v: Value<Outer> = Rc::new(RefCell::new(<Outer>::uninit()));
            ({ make_0(__v.as_pointer()) });
            __v
        }
        .as_pointer()
        {
            __tmp_0 => field_ref!(__tmp_0, values).as_pointer(),
        })
    });
    printf!(b"%d\n", { (*h.borrow()).last });
    return 0;
}
pub trait OuterImpl {
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer>;
}
impl OuterImpl for Ptr<Outer> {
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer> {
        field!((*self), values).with_mut(|__b0| {
            (*__b0) = field!(_a0, values).with_mut(std::mem::take);
        });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
