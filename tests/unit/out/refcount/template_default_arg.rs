extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new_1, value)]
#[byte_size(4)]
pub struct A {
    #[offset(0)]
    pub v: i32,
}
impl A {
    pub fn new_2(this: Ptr<A>) {
        field!(this, v).write(1);
    }
    pub fn new_1(this: Ptr<A>, mut v: i32) {
        field!(this, v).write(v);
    }
}
impl Default for A {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<A> = Rc::new(RefCell::new(<A>::uninit()));
            A::new_2(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct B {
    #[offset(0)]
    pub v: i32,
}
impl B {
    pub fn new(this: Ptr<B>) {
        field!(this, v).write(2);
    }
}
impl Default for B {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<B> = Rc::new(RefCell::new(<B>::uninit()));
            B::new(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct NoDefault {
    #[offset(0)]
    pub v: i32,
}
impl NoDefault {
    pub fn new(this: Ptr<NoDefault>, mut v: i32) {
        field!(this, v).write(v);
    }
}
pub fn used_0(x: Value<A>) -> i32 {
    return { (*x.borrow()).v };
}
pub fn used_1(x: Value<B>) -> i32 {
    return { (*x.borrow()).v };
}
pub fn scaled_2(x: Value<A>, mut n: i32) -> i32 {
    return ({ (*x.borrow()).v } * n);
}
pub fn always_given_3(x: Value<NoDefault>) -> i32 {
    return { (*x.borrow()).v };
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct S_NoDefault_ {
    #[offset(0)]
    pub v: i32,
}
impl S_NoDefault_ {
    pub fn new(this: Ptr<S_NoDefault_>, mut v: i32) {
        field!(this, v).write(v);
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    assert!(
        (({
            used_0({
                let __v: Value<A> = Rc::new(RefCell::new(<A>::uninit()));
                A::new_2(__v.as_pointer());
                __v
            })
        }) == 1)
    );
    assert!(
        (({
            used_0({
                let __v: Value<A> = Rc::new(RefCell::new(<A>::uninit()));
                A::new_1(__v.as_pointer(), { 5 });
                __v
            })
        }) == 5)
    );
    assert!(
        (({
            used_1({
                let __v: Value<B> = Rc::new(RefCell::new(<B>::uninit()));
                B::new(__v.as_pointer());
                __v
            })
        }) == 2)
    );
    assert!(
        (({
            scaled_2(
                {
                    let __v: Value<A> = Rc::new(RefCell::new(<A>::uninit()));
                    A::new_1(__v.as_pointer(), { 3 });
                    __v
                },
                (4usize as i32),
            )
        }) == (3 * (4usize as i32)))
    );
    assert!(
        (({
            scaled_2(
                {
                    let __v: Value<A> = Rc::new(RefCell::new(<A>::uninit()));
                    A::new_1(__v.as_pointer(), { 3 });
                    __v
                },
                2,
            )
        }) == 6)
    );
    assert!(
        (({
            always_given_3({
                let __v: Value<NoDefault> = Rc::new(RefCell::new(<NoDefault>::uninit()));
                NoDefault::new(__v.as_pointer(), { 3 });
                __v
            })
        }) == 3)
    );
    let s: Value<S_NoDefault_> = Rc::new(RefCell::new(<S_NoDefault_>::uninit()));
    S_NoDefault_::new(s.as_pointer(), { 1 });
    assert!(
        (({
            S_NoDefault_Impl::get(&s.as_pointer(), {
                let __v: Value<NoDefault> = Rc::new(RefCell::new(<NoDefault>::uninit()));
                NoDefault::new(__v.as_pointer(), { 4 });
                __v
            })
        }) == 5)
    );
    return 0;
}
pub trait S_NoDefault_Impl {
    fn get(&self, t: Value<NoDefault>) -> i32;
}
impl S_NoDefault_Impl for Ptr<S_NoDefault_> {
    fn get(&self, t: Value<NoDefault>) -> i32 {
        return ((*self).with(|__s| __s.v) + { (*t.borrow()).v });
    }
}
pub fn __cpp2rust_init_globals() {}
