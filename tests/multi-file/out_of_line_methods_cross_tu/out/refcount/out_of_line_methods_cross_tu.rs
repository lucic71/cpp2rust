extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[placement]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub v: i32,
}
impl S {
    pub fn copy_from(this: Ptr<S>, _a0: Ptr<S>) {
        field!(this, v).write(_a0.with(|__s| __s.v));
    }
}
pub trait Base {
    fn apply(&mut self, x: i32) -> i32;
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[placement]
#[byte_size(16)]
pub struct Derived {
    #[offset(8)]
    pub factor: i32,
}
impl Derived {
    pub fn copy_from(this: Ptr<Derived>, _a0: Ptr<Derived>) {
        field!(this, factor).write(_a0.with(|__s| __s.factor));
    }
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub first: i32,
    #[offset(4)]
    pub second: i32,
}
pub fn pair_diff_0(mut p: Ptr<Pair>) -> i32 {
    return ({ p.with(|__s| __s.first) } - { p.with(|__s| __s.second) });
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(<S>::uninit()));
    S::new(s.as_pointer(), { 1 });
    assert!((({ SImpl::get(&s.as_pointer(),) }) == 1));
    ({ SImpl::set(&s.as_pointer(), 4) });
    assert!((({ SImpl::get(&s.as_pointer(),) }) == 4));
    assert!((({ SImpl::add(&s.as_pointer(), 2,) }) == 6));
    let derived: Value<Derived> = Rc::new(RefCell::new(<Derived>::uninit()));
    Derived::new(derived.as_pointer(), { 3 });
    let mut base: PtrDyn<dyn Base> = (derived.as_pointer()).to_dyn::<dyn Base>(|w| w);
    assert!((({ (*base.upgrade().deref_mut()).apply(5,) }) == 15));
    let pair: Value<Pair> = Rc::new(RefCell::new(Pair {
        first: 7,
        second: 3,
    }));
    assert!((({ pair_sum_1((pair.as_pointer()),) }) == 10));
    assert!((({ pair_diff_0((pair.as_pointer()),) }) == 4));
    return 0;
}
impl S {
    pub fn new(this: Ptr<S>, mut x: i32) {
        field!(this, v).write(x);
    }
}
impl Derived {
    pub fn new(this: Ptr<Derived>, mut factor: i32) {
        field!(this, factor).write(factor);
    }
}
impl S {}
impl Derived {}
pub fn pair_sum_1(mut p: Ptr<Pair>) -> i32 {
    return ({ p.with(|__s| __s.first) } + { p.with(|__s| __s.second) });
}
impl Base for Derived {
    fn apply(&mut self, mut x: i32) -> i32 {
        return ({ self.factor } * x);
    }
}
pub trait SImpl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn get(&self) -> i32;
    fn set(&self, mut x: i32) {
        unimplemented!()
    }
    fn add(&self, mut x: i32) -> i32 {
        unimplemented!()
    }
}
impl SImpl for Ptr<S> {
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.v);
    }
    fn destructor(&self) {}
    fn set(&self, mut x: i32) {
        field!((*self), v).write(x);
    }
    fn add(&self, mut x: i32) -> i32 {
        {
            let __rhs = x;
            field!((*self), v).with_mut(|__v| *__v = *__v + __rhs)
        };
        return (*self).with(|__s| __s.v);
    }
}
pub fn __cpp2rust_init_globals() {}
