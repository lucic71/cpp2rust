extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Default)]
#[placement]
#[cpp_from(i32, new_1, value)]
#[byte_size(16)]
pub struct S {
    #[offset(0)]
    pub a_: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub self__: Ptr<S>,
}
impl S {
    pub fn new_1(this: Ptr<S>, mut a: i32) {
        field!(this, a_).write(a);
        field!(this, self__).write(Ptr::<S>::null());
    }
    pub fn new_2(this: Ptr<S>, mut a: i32, mut other: Ptr<S>) {
        field!(this, a_).write(a);
        if (this == other) {
            field!(this, self__).write(Ptr::<S>::null());
        } else {
            field!(this, self__).write((other).clone());
        }
    }
    pub fn new_3(this: Ptr<S>, mut a: i32, mut other: Ptr<S>) {
        field!(this, a_).write(a);
        field!(this, self__).write(Ptr::<S>::null());
        if (this == other) {
            field!(this, self__).write(Ptr::<S>::null());
        }
    }
    pub fn copy_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, a_).write(_a0.with(|__s| __s.a_));
        field!(this, self__).write(_a0.with(|__s| __s.self__.clone()));
    }
}
pub fn bump_0(mut p: Ptr<S>) {
    field!(p, a_).with_mut(|__v| __v.postfix_inc());
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, CppFrom, Default)]
#[placement]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct D {
    #[offset(0)]
    pub a_: i32,
}
impl D {
    pub fn new(this: Ptr<D>, mut a: i32) {
        field!(this, a_).write(a);
        {
            field!(this, a_).with_mut(|__v| *__v = *__v * 2)
        };
    }
    pub fn copy_from(this: Ptr<D>, _a0: Ptr<D>) {
        field!(this, a_).write(_a0.with(|__s| __s.a_));
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    S::new_1(s.as_pointer(), { 1 });
    let ref_: Ptr<S> = ({ SImpl::returns_this_reference(&s.as_pointer()) });
    field!(ref_, a_).with_mut(|__v| __v.postfix_inc());
    assert!(({ ({ (*s.borrow()).a_ } == 2) }));
    let mut ptr: Ptr<S> = ({ SImpl::returns_this_pointer(&s.as_pointer()) });
    field!(ptr, a_).with_mut(|__v| __v.postfix_inc());
    assert!(({ ({ (*s.borrow()).a_ } == 3) }));
    ({ SImpl::inc(&({ SImpl::inc(&({ SImpl::inc(&s.as_pointer()) })) })) });
    assert!(({ ({ (*s.borrow()).a_ } == 6) }));
    ({ SImpl::set_from_this(&s.as_pointer()) });
    assert!(({ ({ (*s.borrow()).a_ } == 7) }));
    assert!((({ SImpl::twice(&s.as_pointer(),) }) == 14));
    ({ SImpl::link(&s.as_pointer()) });
    assert!(({ ({ { (*s.borrow()).self__.clone() } } == { (s.as_pointer()) }) }));
    field!({ (*s.borrow()).self__.clone() }, a_).with_mut(|__v| __v.postfix_inc());
    assert!(({ ({ (*s.borrow()).a_ } == 8) }));
    ({ SImpl::bump_me(&s.as_pointer()) });
    assert!(({ ({ (*s.borrow()).a_ } == 9) }));
    let d: Value<D> = Rc::new(RefCell::new(<D>::uninit()));
    D::new(d.as_pointer(), { 3 });
    assert!(({ ({ (*d.borrow()).a_ } == 6) }));
    let cr: Ptr<S> = ({ SImpl::cref(&s.as_pointer()) });
    assert!((cr.with(|__s| __s.a_) == 9));
    let t: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    S::new_1(t.as_pointer(), { 0 });
    assert!(
        ({
            let _o: Ptr<S> = (s.as_pointer());
            SImpl::is(&s.as_pointer(), _o)
        })
    );
    assert!(!({ SImpl::is(&s.as_pointer(), (t.as_pointer()),) }));
    let mut p: Ptr<S> = {
        let __p = Ptr::alloc(<S>::uninit());
        S::new_1(__p.clone(), { 1 });
        __p
    };
    let mut q: Ptr<S> = ({ SImpl::returns_this_pointer(&p) });
    field!(q, a_).with_mut(|__v| __v.postfix_inc());
    assert!((p.with(|__s| __s.a_) == 2));
    p.delete();
    let mut h: Ptr<S> = {
        let __p = Ptr::alloc(<S>::uninit());
        S::new_1(__p.clone(), { 5 });
        __p
    };
    ({ SImpl::destroy(&h) });
    ({ SImpl::reset(&s.as_pointer()) });
    assert!(({ ({ (*s.borrow()).a_ } == 0) }));
    assert!(({ ({ (*s.borrow()).self__.clone() }).is_null() }));
    assert!(
        ((({
            let _other: Ptr<S> = (s.as_pointer());
            SImpl::copy_if_different(&s.as_pointer(), _other)
        }) as i32)
            == (false as i32))
    );
    assert!(
        ((({
            let _other: Ptr<S> = (s.as_pointer());
            SImpl::copy_if_different_const(&s.as_pointer(), _other)
        }) as i32)
            == (false as i32))
    );
    let other: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    S::new_1(other.as_pointer(), { 22 });
    assert!(
        ((({ SImpl::copy_if_different(&s.as_pointer(), (other.as_pointer()),) }) as i32)
            == (true as i32))
    );
    assert!(({ ({ (*s.borrow()).a_ } == { (*other.borrow()).a_ }) }));
    assert!(
        ({ ({ { (*s.borrow()).self__.clone() } } == { { (*other.borrow()).self__.clone() } }) })
    );
    let u: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    S::new_2(u.as_pointer(), { 1 }, { (s.as_pointer()) });
    assert!(({ ({ { (*u.borrow()).self__.clone() } } == { (s.as_pointer()) }) }));
    let s_const: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    S::new_1(s_const.as_pointer(), { 100 });
    let u1: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    S::new_3(u1.as_pointer(), { 1 }, { (s_const.as_pointer()) });
    assert!(({ ({ (*u1.borrow()).self__.clone() }).is_null() }));
    return 0;
}
pub trait SImpl {
    fn returns_this_reference(&self) -> Ptr<S>;
    fn returns_this_pointer(&self) -> Ptr<S>;
    fn inc(&self) -> Ptr<S>;
    fn set_from_this(&self);
    fn get(&self) -> i32;
    fn twice(&self) -> i32;
    fn link(&self);
    fn bump_me(&self);
    fn cref(&self) -> Ptr<S>;
    fn is(&self, o: Ptr<S>) -> bool;
    fn destroy(&self);
    fn reset(&self);
    fn copy_if_different_const(&self, other: Ptr<S>) -> bool;
    fn copy_if_different(&self, other: Ptr<S>) -> bool;
}
impl SImpl for Ptr<S> {
    fn returns_this_reference(&self) -> Ptr<S> {
        return (*self).clone();
    }
    fn returns_this_pointer(&self) -> Ptr<S> {
        return (*self).clone();
    }
    fn inc(&self) -> Ptr<S> {
        field!((*self), a_).with_mut(|__v| __v.postfix_inc());
        return (*self).clone();
    }
    fn set_from_this(&self) {
        field!((*self), a_).write({ ((*self).with(|__s| __s.a_) + 1) });
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.a_);
    }
    fn twice(&self) -> i32 {
        return (({ SImpl::get(self) }) * 2);
    }
    fn link(&self) {
        field!((*self), self__).write((*self).clone());
    }
    fn bump_me(&self) {
        ({ bump_0((*self).clone()) });
    }
    fn cref(&self) -> Ptr<S> {
        return (*self).clone();
    }
    fn is(&self, mut o: Ptr<S>) -> bool {
        return (o == (*self));
    }
    fn destroy(&self) {
        (*self).delete();
    }
    fn reset(&self) {
        (*self).write(
            Rc::try_unwrap({
                let __v: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
                S::new_1(__v.as_pointer(), { 0 });
                __v
            })
            .ok()
            .unwrap()
            .into_inner(),
        );
    }
    fn copy_if_different_const(&self, mut other: Ptr<S>) -> bool {
        if ((*self) == other) {
            return false;
        }
        field!((*self), a_).write({ other.with(|__s| __s.a_) });
        field!((*self), self__).write({ other.with(|__s| __s.self__.clone()) });
        return true;
    }
    fn copy_if_different(&self, mut other: Ptr<S>) -> bool {
        if ((*self) == other) {
            return false;
        }
        field!((*self), a_).write({ other.with(|__s| __s.a_) });
        field!((*self), self__).write({ other.with(|__s| __s.self__.clone()) });
        return true;
    }
}
pub fn __cpp2rust_init_globals() {}
