extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(x: ()) -> () {
    let x: Value<()> = Rc::new(RefCell::new(x));
    return (*x.borrow());
}
pub fn foo_1(mut x: Ptr<()>) -> () {
    return (x.read());
}
pub fn foo_2(mut x: Ptr<()>, mut y: Ptr<()>) -> () {
    return ({ (x.read()) } + { (y.read()) });
}
pub fn foo_3(mut x: Ptr<()>, mut y: Ptr<()>, z: Ptr<()>) -> () {
    return ({ ({ (x.read()) } + { (y.read()) }) } + { (z.read()) });
}
pub fn bar_4(x: Ptr<()>) -> () {
    return (x.read());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Foo {}
pub fn func_5(mut x: ()) -> () {
    return 1_();
}
pub fn func_6(mut x: Ptr<()>) -> () {
    return 1_();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let x: Value<()> = Rc::new(RefCell::new(1_()));
    let mut out: () = 0_();
    out += ({ foo_0(0_()) });
    out += ({ foo_1((x.as_pointer())) });
    out += ({ bar_4(x.as_pointer()) });
    out += ({
        let _x: Ptr<()> = (x.as_pointer());
        let _y: Ptr<()> = (x.as_pointer());
        let _z: Ptr<()> = x.as_pointer();
        foo_3(_x, _y, _z)
    });
    out += ({
        let _x: Ptr<()> = (x.as_pointer());
        let _y: Ptr<()> = (x.as_pointer());
        foo_2(_x, _y)
    });
    let mut bar: () = 5_();
    out += ((bar + ({ foo_0(0_()) })) + ({ foo_1((x.as_pointer())) }));
    let foo1: Value<Foo> = Rc::new(RefCell::new(<Foo>::default()));
    let foo2: Value<Foo> = Rc::new(RefCell::new(<Foo>::default()));
    ({ FooImpl::foo_2(&foo1.as_pointer()) });
    ({ FooImpl::method_3(&foo1.as_pointer(), 1_()) });
    ({ FooImpl::foo_1(&foo2.as_pointer()) });
    ({ FooImpl::method_4(&foo2.as_pointer(), 2_()) });
    unimplemented!();
    return 0_();
}
pub trait FooImpl {
    fn foo_1(&self);
    fn foo_2(&self);
    fn method_3(&self, x: ());
    fn method_4(&self, x: ());
    fn method2_5(&self, x: (), y: ());
    fn method2_6(&self, x: (), y: ());
}
impl FooImpl for Ptr<Foo> {
    fn foo_1(&self) {}
    fn foo_2(&self) {}
    fn method_3(&self, mut x: ()) {}
    fn method_4(&self, mut x: ()) {}
    fn method2_5(&self, mut x: (), mut y: ()) {}
    fn method2_6(&self, mut x: (), mut y: ()) {}
}
pub fn __cpp2rust_init_globals() {}
