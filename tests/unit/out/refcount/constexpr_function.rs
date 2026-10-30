extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn runtime_only_0(mut x: ()) -> () {
    return (x * 2_());
}
pub fn first_1(mut p: Ptr<()>) -> () {
    return (p.read());
}
pub fn scaled_2(x: ()) -> () {
    let x: Value<()> = Rc::new(RefCell::new(x));
    if ((*x.borrow()) < 0_()) {
        return ({ runtime_only_0(-(*x.borrow())) });
    }
    return (*x.borrow());
}
pub fn half_3(mut x: ()) -> () {
    return (x / 2_f64);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Flag {
    #[offset(0)]
    pub v: (),
}
pub fn use_4(f: Flag) -> () {
    let f: Value<Flag> = Rc::new(RefCell::new(f));
    unimplemented!();
    return { (*f.borrow()).v };
}
pub fn checked_5(mut x: ()) -> () {
    unimplemented!();
    return (x + 1_());
}
pub fn in_constant_context_6() -> () {
    {}
    return (({ runtime_only_0(1_()) }) == 0_());
}
const _: () = assert!(true, "in_constant_context()");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct P {
    #[offset(0)]
    pub v: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut runtime_context: () = ({ in_constant_context_6() });
    unimplemented!();
    let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([7_(), 8_()])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut c: () = ({ checked_5(4_()) });
    unimplemented!();
    unimplemented!();
    let mut u: () = ({ use_4(Flag { v: 3_() }) });
    unimplemented!();
    let mut ptr: Ptr<()> = (arr.as_pointer() as Ptr<()>);
    unimplemented!();
    let p: Value<P> = Rc::new(RefCell::new(P { v: 9_() }));
    unimplemented!();
    let mut k: () = ({ scaled_2(4_()) });
    unimplemented!();
    return 0_();
}
pub trait FlagImpl {
    fn to_(&self) -> ();
}
impl FlagImpl for Ptr<Flag> {
    fn to_(&self) -> () {
        return ((*self).with(|__s| __s.v) != 0_());
    }
}
pub trait PImpl {
    fn get(&self) -> ();
}
impl PImpl for Ptr<P> {
    fn get(&self) -> () {
        return (*self).with(|__s| __s.v);
    }
}
pub fn __cpp2rust_init_globals() {}
