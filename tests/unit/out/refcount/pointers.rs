extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Test {
    #[offset(0)]
    pub x: (),
}
pub fn Update_0(t: Ptr<Test>) -> Ptr<Test> {
    let t: Value<Ptr<Test>> = Rc::new(RefCell::new(t));
    let mut x: () = 1_();
    let mut y: () = 2_();
    x.prefix_inc();
    ({ TestImpl::update(&(*t.borrow()), x, y) });
    x = (*t.borrow()).with(|__s| (__s).x);
    y = (*t.borrow()).with(|__s| __s.x);
    ({
        let _x: () = x;
        let _y: () = y;
        TestImpl::update(&(*t.borrow()), _x, _y)
    });
    return (*t.borrow()).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let t1: Value<Test> = Rc::new(RefCell::new(Test { x: 100_() }));
    let mut t2: Ptr<Test> = ({ Update_0((t1.as_pointer())) });
    let mut t3: Ptr<Test> = Ptr::<Test>::null();
    t3 = (t2).clone();
    field!(t3, x).write(15_());
    {
        ({ TestImpl::as_ptr(&t3) }).with_mut(|__v| *__v = *__v + 10_())
    };
    unimplemented!();
    return 0_();
}
pub trait TestImpl {
    fn inc(&self);
    fn dec(&self);
    fn as_ptr(&self) -> Ptr<()>;
    fn update(&self, x: (), y: ());
}
impl TestImpl for Ptr<Test> {
    fn inc(&self) {
        field!((*self), x).with_mut(|__v| __v.postfix_inc());
    }
    fn dec(&self) {
        field!((*self), x).with_mut(|__v| __v.postfix_dec());
    }
    fn as_ptr(&self) -> Ptr<()> {
        return (field_ptr!((*self), x));
    }
    fn update(&self, mut x: (), mut y: ()) {
        field!((*self), x).write((x + y));
    }
}
pub fn __cpp2rust_init_globals() {}
