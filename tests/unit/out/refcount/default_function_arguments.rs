extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(mut a: (), b: Option<()>) -> () {
    let mut b: () = b.unwrap_or_else(|| 10_());
    return (a + b);
}
pub fn baz_1(mut a: Ptr<()>, b: Option<Ptr<()>>) -> () {
    let mut b: Ptr<()> = b.unwrap_or_else(|| Ptr::<()>::null());
    return ({ a } == { b });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct Bar {
    #[offset(0)]
    pub v: (),
}
impl Bar {
    pub fn new(v: Option<()>) -> Self {
        let mut v: () = v.unwrap_or_else(|| 1_());
        Self { v: v }
    }
}
impl Default for Bar {
    fn default() -> Self {
        { Bar::new(None) }
    }
}
thread_local!(
    pub static counter_2: Value<()> = Rc::new(RefCell::new(0_()));
);
pub fn next_3() -> () {
    return (*counter_2.with(Value::clone).borrow_mut()).prefix_inc();
}
pub fn lazy_4(x: Option<()>) -> () {
    let x: Value<()> = Rc::new(RefCell::new(x.unwrap_or_else(|| ({ next_3() }))));
    return (*x.borrow());
}
pub fn by_ref_5(b: Option<Ptr<Bar>>) -> () {
    let mut __b_default: Option<Value<Bar>> = None;
    let b: Ptr<Bar> = b.unwrap_or_else(|| {
        __b_default
            .insert(Rc::new(RefCell::new(Bar::new({ Some(7_()) }))))
            .as_pointer()
    });
    return b.with(|__s| __s.v);
}
thread_local!(
    pub static global_bar_6: Value<Bar> = Rc::new(RefCell::new(Bar::new({ Some(9_()) })));
);
pub fn by_global_ref_7(b: Option<Ptr<Bar>>) -> () {
    let b: Ptr<Bar> = b.unwrap_or_else(|| global_bar_6.with(|v| v.as_pointer()));
    return b.with(|__s| __s.v);
}
pub fn shifted_8(mut x: (), y: Option<()>) -> () {
    let mut y: () = y.unwrap_or_else(|| -1_());
    return (x + y);
}
pub fn first_9(r: Ptr<()>) -> () {
    return (elem!((r), 0_()).read());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Holder_int_ {
    #[offset(0)]
    pub v: (),
}
impl Holder_int_ {
    pub fn new(mut v: ()) -> Self {
        Self { v: v }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    let a: Value<()> = Rc::new(RefCell::new(0_()));
    unimplemented!();
    unimplemented!();
    let mut b: Bar = Bar::new(None);
    unimplemented!();
    unimplemented!();
    let mut arr: [Bar; 3] = [Bar::new(None), Bar::new(None), Bar::new(None)];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut h: Holder_int_ = Holder_int_::new({ 4_() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = counter_2.with(|_| ());
    let _ = global_bar_6.with(|_| ());
}
