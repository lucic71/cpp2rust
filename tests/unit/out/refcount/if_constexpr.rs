extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn classify_0(mut x: Ptr<()>) -> () {
    {
        return (x.read());
    }
    return 1_();
}
pub fn classify_1(mut x: ()) -> () {
    {
        {
            return 2_();
        }
    }
    return 1_();
}
pub fn classify_2(mut x: ()) -> () {
    {
        {}
    }
    return 1_();
}
pub fn keep_both_3(mut x: ()) -> () {
    {
        return (x + 1_());
    }
    panic!("ub: non-void function does not return a value")
}
pub fn widen_4(v: ()) -> () {
    let v: Value<()> = Rc::new(RefCell::new(v));
    {}
    return (*v.borrow());
}
pub fn widen_5(v: ()) -> () {
    let v: Value<()> = Rc::new(RefCell::new(v));
    {
        let hi: Value<()> = Rc::new(RefCell::new((*v.borrow())));
        (*hi.borrow_mut()) <<= 32_();
        return (*hi.borrow());
    }
    return (*v.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let v: Value<()> = Rc::new(RefCell::new(7_()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
