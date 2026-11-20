extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(i8, to_i8)]
#[cpp_into(u8, to_u8)]
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
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_into(i64, to_i64_1)]
#[byte_size(4)]
pub struct T {
    #[offset(0)]
    pub v: i32,
}
impl T {
    pub fn new_1(this: Ptr<T>) {}
    pub fn copy_from(this: Ptr<T>, _a0: Ptr<T>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
    pub fn move_from(this: Ptr<T>, _a0: Ptr<T>) {
        field!(this, v).write({ (*_a0.upgrade().deref()).v });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    {
        let __d = s.as_pointer();
        field!(__d, v).write(65);
    };
    let mut a: i8 = ({ SImpl::to_i8(&s.as_pointer()) });
    let mut b: u8 = ({ SImpl::to_u8(&s.as_pointer()) });
    assert!(((a as i32) == (('A' as i8) as i32)));
    assert!(((b as i32) == ((66 as u8) as i32)));
    assert!(
        (((({ SImpl::to_i8(&s.as_pointer(),) }) as i32)
            + (({ SImpl::to_u8(&s.as_pointer(),) }) as i32))
            == 131)
    );
    let t: Value<T> = Rc::new(RefCell::new(<T>::uninit()));
    {
        let __d = t.as_pointer();
        field!(__d, v).write(3);
    };
    let mut c: i64 = ({ TImpl::to_i64_1(&t.as_pointer()) });
    let mut d: i64 = ({ TImpl::to_i64_2(&t.as_pointer()) });
    assert!((c == 3_i64));
    assert!((d == 4_i64));
    return 0;
}
pub trait SImpl {
    fn to_i8(&self) -> i8;
    fn to_u8(&self) -> u8;
}
impl SImpl for Ptr<S> {
    fn to_i8(&self) -> i8 {
        return ((*self).with(|__s| __s.v) as i8);
    }
    fn to_u8(&self) -> u8 {
        return (((*self).with(|__s| __s.v) + 1) as u8);
    }
}
pub trait TImpl {
    fn to_i64_1(&self) -> i64;
    fn to_i64_2(&self) -> i64;
}
impl TImpl for Ptr<T> {
    fn to_i64_1(&self) -> i64 {
        return ((*self).with(|__s| __s.v) as i64);
    }
    fn to_i64_2(&self) -> i64 {
        return (((*self).with(|__s| __s.v) + 1) as i64);
    }
}
pub fn __cpp2rust_init_globals() {}
