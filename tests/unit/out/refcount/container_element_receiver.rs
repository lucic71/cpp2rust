extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(32)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(8)]
    pub ref_: Ptr<()>,
    #[offset(8)]
    #[byte_size(24)]
    pub vec_: (),
}
impl Holder {
    pub fn move_from(_a0: Ptr<Holder>) -> Self {
        Self {
            ref_: ({ (*_a0.upgrade().deref()).ref_.clone() }).clone(),
            vec_: unimplemented!(),
        }
    }
}
pub fn pick_0(a: Ptr<()>, b: Ptr<()>, mut first: ()) -> Ptr<()> {
    return if first { (a).clone() } else { (b).clone() };
}
pub fn sum_1(v: Ptr<()>) -> () {
    let mut total: () = 0_();
    let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    'loop_: while !(unimplemented!()) {
        total += (unimplemented!().read());
        unimplemented!();
    }
    return ({ total } + { (unimplemented!().read()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let h: Value<Holder> = Rc::new(RefCell::new(Holder {
        ref_: v.as_pointer(),
        vec_: unimplemented!(),
    }));
    unimplemented!();
    unimplemented!().write(9_());
    unimplemented!();
    unimplemented!().write(1_());
    unimplemented!();
    let mut p: Ptr<()> = (field_ptr!(h.as_pointer(), vec_));
    unimplemented!();
    unimplemented!().write(7_());
    unimplemented!();
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let c: Ptr<()> = v.as_pointer();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
