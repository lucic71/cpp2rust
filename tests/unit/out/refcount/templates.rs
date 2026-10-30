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
pub fn foo_1(x: ()) -> () {
    let x: Value<()> = Rc::new(RefCell::new(x));
    return (*x.borrow());
}
pub fn pick_2() -> () {
    return 0_();
}
pub fn pick_3() -> () {
    return 1_();
}
pub fn bar_4(mut p: Ptr<()>, mut flag: ()) -> Ptr<()> {
    return if flag { p } else { Ptr::<()>::null() };
}
pub fn bar_5(mut p: Ptr<()>, mut flag: ()) -> Ptr<()> {
    return if flag { p } else { Ptr::<()>::null() };
}
pub fn func_6(mut x1: (), mut x2: (), mut x3: ()) -> () {
    return ((x1 + x2) + x3);
}
pub fn func_7(mut x1: (), mut x2: (), mut x3: ()) -> () {
    return ((x1 + x2) + x3);
}
thread_local!(
    pub static half_8: Value<()> = Rc::new(RefCell::new((1_() / 2_())));
);
thread_local!(
    pub static half_9: Value<()> = Rc::new(RefCell::new((1_() / 2_())));
);
thread_local!(
    pub static half_10: Value<Ptr<()>> = Rc::new(RefCell::new(Ptr::<()>::null()));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let x: Value<()> = Rc::new(RefCell::new(10_()));
    let y: Value<()> = Rc::new(RefCell::new((*x.borrow())));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    half_8.with(|rc| *rc.borrow_mut() = 7_());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    half_10.with(|rc| *rc.borrow_mut() = (x.as_pointer()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = half_8.with(|_| ());
    let _ = half_9.with(|_| ());
    let _ = half_10.with(|_| ());
}
