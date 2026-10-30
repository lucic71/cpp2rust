extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn identity_0(x: ()) -> () {
    let x: Value<()> = Rc::new(RefCell::new(x));
    return (*x.borrow());
}
pub fn swap_by_ptr_1(mut a: Ptr<()>, mut b: Ptr<()>) {
    let mut tmp: () = (a.read());
    a.write({ (b.read()) });
    b.write({ tmp });
}
pub fn swap_by_ref_2(a: Ptr<()>, b: Ptr<()>) {
    let mut tmp: () = (a.read());
    a.write({ (b.read()) });
    b.write({ tmp });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut local: () = 0_();
    let a: Value<()> = Rc::new(RefCell::new(1_()));
    let b: Value<()> = Rc::new(RefCell::new(2_()));
    let c: Value<()> = Rc::new(RefCell::new(({ identity_0(local) })));
    let mut p: Ptr<()> = (a.as_pointer());
    p = (b.as_pointer());
    p = (a.as_pointer());
    ({ swap_by_ptr_1((p).clone(), (b.as_pointer())) });
    ({ swap_by_ref_2(a.as_pointer(), c.as_pointer()) });
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
