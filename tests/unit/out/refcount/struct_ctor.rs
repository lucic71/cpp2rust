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
pub struct StructWithCtor {
    #[offset(0)]
    x1_: i32,
    #[offset(4)]
    x2_: i32,
}
impl StructWithCtor {
    pub fn new(this: Ptr<StructWithCtor>, mut x1: i32, mut x2: i32) {
        field!(this, x1_).write(x1);
        field!(this, x2_).write(x2);
        field!(this, x1_).with_mut(|__v| {
            __v.prefix_inc();
        });
        field!(this, x2_).with_mut(|__v| {
            __v.prefix_dec();
        });
    }
    pub fn copy_from(this: Ptr<StructWithCtor>, _a0: Ptr<StructWithCtor>) {
        field!(this, x1_).write(_a0.with(|__s| __s.x1_));
        field!(this, x2_).write(_a0.with(|__s| __s.x2_));
    }
    pub fn move_from(this: Ptr<StructWithCtor>, _a0: Ptr<StructWithCtor>) {
        field!(this, x1_).write({ (*_a0.upgrade().deref()).x1_ });
        field!(this, x2_).write({ (*_a0.upgrade().deref()).x2_ });
    }
}
pub fn foo_0(x: Ptr<i32>) -> Ptr<i32> {
    return (x).clone();
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct Value_ {
    #[offset(0)]
    pub v: i32,
}
impl Value_ {
    pub fn new(this: Ptr<Value_>, mut u: i32) {
        field!(this, v).write(u);
    }
    pub fn copy_from(this: Ptr<Value_>, _a0: Ptr<Value_>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
    pub fn move_from(this: Ptr<Value_>, _a0: Ptr<Value_>) {
        field!(this, v).write({ (*_a0.upgrade().deref()).v });
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Ptr_ {
    #[offset(0)]
    #[byte_size(4)]
    pub v1: Value_,
    #[offset(4)]
    #[byte_size(4)]
    pub v2: Value_,
}
impl Ptr_ {
    pub fn new(this: Ptr<Ptr_>) {
        Value_::new(field_ptr!(this, v1), { 11 });
        Value_::new(field_ptr!(this, v2), { 22 });
    }
    pub fn copy_from(this: Ptr<Ptr_>, _a0: Ptr<Ptr_>) {
        Value_::copy_from(field_ptr!(this, v1), { field_ptr!(_a0, v1) });
        Value_::copy_from(field_ptr!(this, v2), { field_ptr!(_a0, v2) });
    }
    pub fn move_from(this: Ptr<Ptr_>, _a0: Ptr<Ptr_>) {
        Value_::move_from(field_ptr!(this, v1), { field_ptr!(_a0, v1) });
        Value_::move_from(field_ptr!(this, v2), { field_ptr!(_a0, v2) });
    }
}
impl Default for Ptr_ {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<Ptr_> = Rc::new(RefCell::new(<Ptr_>::uninit()));
            Ptr_::new(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let struct_with_ctor: Value<StructWithCtor> = Rc::new(RefCell::new(<StructWithCtor>::uninit()));
    StructWithCtor::new(struct_with_ctor.as_pointer(), { 1 }, { 2 });
    let x: Value<i32> = Rc::new(RefCell::new(3));
    assert!(
        (((({ foo_0(x.as_pointer(),) }).read()) == 3)
            && ((({ StructWithCtorImpl::x1(&struct_with_ctor.as_pointer(),) }).read()) == 2))
            && ((({ StructWithCtorImpl::x2(&struct_with_ctor.as_pointer(),) }).read()) == 1)
    );
    let p: Value<Ptr_> = Rc::new(RefCell::new(<Ptr_>::uninit()));
    Ptr_::new(p.as_pointer());
    assert!(({ ({ (*p.borrow()).v1.v } == 11) }));
    assert!(({ ({ (*p.borrow()).v2.v } == 22) }));
    return 0;
}
pub trait StructWithCtorImpl {
    fn x1(&self) -> Ptr<i32>;
    fn x2(&self) -> Ptr<i32>;
}
impl StructWithCtorImpl for Ptr<StructWithCtor> {
    fn x1(&self) -> Ptr<i32> {
        return field_ptr!((*self), x1_);
    }
    fn x2(&self) -> Ptr<i32> {
        return field_ptr!((*self), x2_);
    }
}
pub fn __cpp2rust_init_globals() {}
