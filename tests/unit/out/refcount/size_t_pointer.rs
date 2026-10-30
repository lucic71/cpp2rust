extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn out_param_0(out: Option<Ptr<()>>) {
    let mut out: Ptr<()> = out.unwrap_or_else(|| Ptr::<()>::null());
    if !(out).is_null() {
        out.write(4_());
    }
}
pub fn parse_1(v: (), idx: Option<Ptr<()>>) -> () {
    let v: Value<()> = Rc::new(RefCell::new(v));
    let mut idx: Ptr<()> = idx.unwrap_or_else(|| Ptr::<()>::null());
    if !(idx).is_null() {
        idx.write(3_());
    }
    return (*v.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let v2: Value<()> = Rc::new(RefCell::new(0_()));
    ({ out_param_0(Some((v2.as_pointer()))) });
    unimplemented!();
    let pidx: Value<()> = Rc::new(RefCell::new(0_()));
    unimplemented!();
    unimplemented!();
    let sv: Value<()> = Rc::new(RefCell::new(7_()));
    let sp: Value<Ptr<()>> = Rc::new(RefCell::new((sv.as_pointer())));
    let mut spp: Ptr<Ptr<()>> = (sp.as_pointer());
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
