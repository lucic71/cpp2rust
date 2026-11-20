extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_0(n: i32, __args: Box<[VaArg]>) -> i32 {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*ap.borrow_mut()) = __args.decay();
    let guard: Value<Guard_1> = Rc::new(RefCell::new(<Guard_1>::uninit()));
    Guard_1::new(guard.as_pointer(), { ap.as_pointer() });
    let _dtor_guard = std::mem::DropGuard::new(guard.clone(), |__p| __p.as_pointer().destructor());
    let mut total: i32 = 0;
    {
        let mut i: i32 = 0;
        'loop_: while (i < (*n.borrow())) {
            total += { (*guard.borrow()).ap.clone() }.with_mut(|__v| __v.arg::<i32>());
            i.prefix_inc();
        }
    }
    return total;
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Destructor, Default)]
#[cpp_from(Ptr<VaArg> , new, mut_ptr)]
#[byte_size(16)]
pub struct Guard_1 {
    #[offset(0)]
    #[byte_size(8)]
    pub ap: Ptr<Ptr<VaArg>>,
    #[offset(8)]
    pub active: bool,
}
impl Guard_1 {
    pub fn new(this: Ptr<Guard_1>, val: Ptr<Ptr<VaArg>>) {
        field!(this, ap).write((val).clone());
        field!(this, active).write(true);
    }
    pub fn copy_from(this: Ptr<Guard_1>, _a0: Ptr<Guard_1>) {
        field!(this, ap).write((_a0.with(|__s| __s.ap.clone())).clone());
        field!(this, active).write(_a0.with(|__s| __s.active));
    }
}
pub fn sum_ptr_2(n: i32, __args: Box<[VaArg]>) -> i32 {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let n: Value<i32> = Rc::new(RefCell::new(n));
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*ap.borrow_mut()) = __args.decay();
    let cursor: Value<Cursor_3> = Rc::new(RefCell::new(<Cursor_3>::uninit()));
    {
        let __d = cursor.as_pointer();
        field!(__d, ap).write((ap.as_pointer()));
    };
    let mut total: i32 = 0;
    {
        let mut i: i32 = 0;
        'loop_: while (i < (*n.borrow())) {
            total += { (*cursor.borrow()).ap.clone() }.with_mut(|__v| __v.arg::<i32>());
            i.prefix_inc();
        }
    };
    return total;
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Cursor_3 {
    #[offset(0)]
    #[byte_size(8)]
    pub ap: Ptr<Ptr<VaArg>>,
}
impl Cursor_3 {
    pub fn new_1(this: Ptr<Cursor_3>) {}
    pub fn copy_from(this: Ptr<Cursor_3>, _a0: Ptr<Cursor_3>) {
        field!(this, ap).write(_a0.with(|__s| __s.ap.clone()));
    }
    pub fn move_from(this: Ptr<Cursor_3>, _a0: Ptr<Cursor_3>) {
        field!(this, ap).write({ (*_a0.upgrade().deref()).ap.clone() });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ sum_0(3, Box::new([(1).into(), (2).into(), (3).into(),])) }) == 6));
    assert!((({ sum_ptr_2(2, Box::new([(4).into(), (5).into(),])) }) == 9));
    return 0;
}
pub trait Guard_1Impl {
    fn destructor(&self);
}
impl Guard_1Impl for Ptr<Guard_1> {
    fn destructor(&self) {
        if (*self).with(|__s| __s.active) {}
    }
}
pub fn __cpp2rust_init_globals() {}
