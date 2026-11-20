extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, CppFrom, Default)]
#[placement]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct Chain {
    #[offset(0)]
    pub v: i32,
}
impl Chain {
    pub fn new(this: Ptr<Chain>, mut v: i32) {
        field!(this, v).write(v);
    }
    pub fn copy_from(this: Ptr<Chain>, o: Ptr<Chain>) {
        field!(this, v).write((o.with(|__s| __s.v) + 100));
    }
    pub fn move_from(this: Ptr<Chain>, o: Ptr<Chain>) {
        field!(this, v).write((o.with(|__s| __s.v) + 1));
        field!(o, v).write(0);
    }
}
impl Clone for Chain {
    fn clone(&self) -> Self {
        let __src: Value<Chain> = Rc::new(RefCell::new(Chain { v: self.v.clone() }));
        let __dst: Value<Chain> = Rc::new(RefCell::new(<Chain>::uninit()));
        Chain::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
pub fn consume_0(c: Value<Chain>) -> i32 {
    return { (*c.borrow()).v };
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Chain> = Rc::new(RefCell::new(<Chain>::uninit()));
    Chain::new(a.as_pointer(), { 1 });
    ({ ChainImpl::add_4(&({ ChainImpl::add_4(&a.as_pointer(), 1) }), 1) });
    assert!(({ ({ (*a.borrow()).v } == 3) }));
    let b0: Value<Chain> = Rc::new(RefCell::new(<Chain>::uninit()));
    Chain::new(b0.as_pointer(), { 5 });
    let b: Value<Chain> = Rc::new(RefCell::new(<Chain>::uninit()));
    Chain::move_from(b.as_pointer(), {
        ({ ChainImpl::add_5(&({ ChainImpl::add_5(&b0.as_pointer(), 1) }), 1) })
    });
    assert!(({ ({ (*b.borrow()).v } == 8) && ({ (*b0.borrow()).v } == 0) }));
    let c: Value<Chain> = Rc::new(RefCell::new(<Chain>::uninit()));
    ({
        ChainImpl::take(
            &{
                let __v: Value<Chain> = Rc::new(RefCell::new(<Chain>::uninit()));
                Chain::new(__v.as_pointer(), { 10 });
                __v
            }
            .as_pointer(),
            c.as_pointer(),
        )
    });
    assert!(({ ({ (*c.borrow()).v } == 11) }));
    let d: Value<Chain> = Rc::new(RefCell::new(<Chain>::uninit()));
    ({ ChainImpl::copy(&c.as_pointer(), d.as_pointer()) });
    assert!(({ ({ (*d.borrow()).v } == 111) && ({ (*c.borrow()).v } == 11) }));
    let g: Value<Chain> = Rc::new(RefCell::new(<Chain>::uninit()));
    Chain::new(g.as_pointer(), { 20 });
    assert!(
        (({
            consume_0({
                let __v: Value<Chain> = Rc::new(RefCell::new(<Chain>::uninit()));
                Chain::move_from(__v.as_pointer(), {
                    ({ ChainImpl::self_(&g.as_pointer()) })
                });
                __v
            })
        }) == 21)
    );
    let e: Value<Chain> = Rc::new(RefCell::new(<Chain>::uninit()));
    Chain::new(e.as_pointer(), { 30 });
    let f: Value<Chain> = Rc::new(RefCell::new(<Chain>::uninit()));
    ({ ChainImpl::take(&e.as_pointer(), f.as_pointer()) });
    assert!(({ ({ (*f.borrow()).v } == 31) && ({ (*e.borrow()).v } == 0) }));
    return 0;
}
pub trait ChainImpl {
    fn add_4(&self, n: i32) -> Ptr<Chain>;
    fn add_5(&self, n: i32) -> Ptr<Chain>;
    fn take(&self, __ret: Ptr<Chain>);
    fn copy(&self, __ret: Ptr<Chain>);
    fn self_(&self) -> Ptr<Chain>;
}
impl ChainImpl for Ptr<Chain> {
    fn add_4(&self, mut n: i32) -> Ptr<Chain> {
        {
            let __rhs = n;
            field!((*self), v).with_mut(|__v| *__v = *__v + __rhs)
        };
        return (*self).clone();
    }
    fn add_5(&self, mut n: i32) -> Ptr<Chain> {
        {
            let __rhs = n;
            field!((*self), v).with_mut(|__v| *__v = *__v + __rhs)
        };
        return (*self).clone();
    }
    fn take(&self, __ret: Ptr<Chain>) {
        Chain::move_from(__ret.clone(), { (*self).clone() });
        return;
    }
    fn copy(&self, __ret: Ptr<Chain>) {
        Chain::copy_from(__ret.clone(), { (*self).clone() });
        return;
    }
    fn self_(&self) -> Ptr<Chain> {
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
